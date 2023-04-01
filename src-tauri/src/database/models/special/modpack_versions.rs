use std::{error::Error, fs, path::Path};

use reqwest::Client;
use rusqlite::{params, Connection};

use crate::{
    database::models::{ModVersion, Modpack, ModpackVersion, Saved, Settings},
    files,
};

impl ModpackVersion {
    pub async fn create_from_modpack(
        modpack: Modpack<Saved>,
        client: &Client,
        db: &mut Connection,
        game_version: &str,
    ) -> Result<ModpackVersion<Saved>, Box<dyn Error>> {
        let id = modpack.id.expect("Saved modpack should have an id");

        // Get all ModVersions from modpack's list of mods and save them
        for mod1 in &modpack.mods {
            ModVersion::get_from_mod(mod1, client, db, game_version, None).await?;
        }

        Ok(ModpackVersion::new(id, game_version.to_string()).save(db)?)
    }

    pub fn load(
        modpack_version_id: i64,
        app_handle: &tauri::AppHandle,
        db: &mut Connection,
    ) -> Result<(), Box<dyn Error>> {
        let load_modpack_version = include_str!("../../../../sql/modpack_versions/load.sql");

        let tx = db.transaction()?;

        Self::unload_all(&tx)?;

        // Check if the version has been downloaded
        if !ModpackVersion::is_installed(modpack_version_id, app_handle) {
            return Err("Cannot load modpack version that hasn't been downloaded".into());
        }

        // Get the modpack version directory
        let mut version_dir = files::get_data_path(app_handle);
        let dir_name = format!("{}/", modpack_version_id);
        version_dir.push(dir_name);

        // Get the minecraft mods directory
        let mut mods_dir = Settings::get_mc_dir(&tx)?;
        mods_dir.push("mods");

        // Create the original_mods directory
        mods_dir.push("original_mods");
        fs::create_dir_all(&mods_dir)?;
        mods_dir.pop();

        // Move all files in mods dir to mods/original_mods dir
        let mods_files = mods_dir.read_dir().expect("Should read mods directory");
        for file in mods_files {
            let file = file?;
            if file.metadata()?.file_type().is_dir() {
                continue;
            }

            let file_name = file.file_name().into_string().unwrap();
            let mut new_path = mods_dir.clone();
            new_path.push("original_mods");
            new_path.push(file_name);
            fs::rename(file.path(), new_path)?;
        }

        // Copy files from modpack version dir to mods dir
        let version_files = version_dir
            .read_dir()
            .expect("Should read version directory");
        for file in version_files {
            let file = file?;
            if file.metadata()?.file_type().is_dir() {
                continue;
            }

            let file_name = file.file_name().into_string().unwrap();
            let mut new_path = mods_dir.clone();
            new_path.push(file_name);
            fs::copy(file.path(), new_path)?;
        }

        tx.execute(load_modpack_version, params![modpack_version_id])?;

        tx.commit()?;

        Ok(())
    }

    pub fn unload_all(db: &Connection) -> Result<(), Box<dyn Error>> {
        let unload_all_modpack_versions =
            include_str!("../../../../sql/modpack_versions/unload_all.sql");

        db.execute(unload_all_modpack_versions, [])?;

        // Get the minecraft mods directory
        let mut mods_dir = Settings::get_mc_dir(&db)?;
        mods_dir.push("mods");

        // Get the original_mods directory
        let mut original_mods_dir = mods_dir.clone();
        original_mods_dir.push("original_mods");

        if Path::new(&original_mods_dir).exists() {
            // Remove all files in mods dir
            let mods_files = mods_dir.read_dir().expect("Should read mods directory");
            for file in mods_files {
                let file = file?;
                if file.metadata()?.file_type().is_dir() {
                    continue;
                }

                fs::remove_file(file.path())?;
            }

            // Move all files in mods/original_mods dir to mods dir
            let mods_files = original_mods_dir.read_dir()?;
            for file in mods_files {
                let file = file?;
                if file.metadata()?.file_type().is_dir() {
                    continue;
                }

                let file_name = file.file_name().into_string().unwrap();
                let mut new_path = original_mods_dir.clone();
                new_path.pop();
                new_path.push(file_name);
                fs::rename(file.path(), new_path)?;
            }

            // Remove the original_mods directory
            fs::remove_dir_all(&original_mods_dir)?;
        }

        Ok(())
    }
}
