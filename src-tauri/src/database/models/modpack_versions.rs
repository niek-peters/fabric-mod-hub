use derive_new::new;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::{error::Error, fs, marker::PhantomData, path::Path};

use crate::{database::errors, files};

use super::{ModVersion, NotSaved, Saved};

#[derive(new, Serialize)]
pub struct ModpackVersion<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub modpack_id: i64,
    pub game_version: String,
    #[new(default)]
    pub installed: bool,
    #[new(default)]
    pub loaded: bool,
    state: PhantomData<State>,
}

impl ModpackVersion<NotSaved> {
    pub fn save(self, db: &mut Connection) -> Result<ModpackVersion<Saved>, Box<dyn Error>> {
        let create_modpack_version = include_str!("../../../sql/modpack_versions/create.sql");

        let tx = db.transaction()?;

        let id = match tx.execute(
            create_modpack_version,
            params![self.modpack_id, self.game_version],
        ) {
            Ok(_) => tx.last_insert_rowid(),
            Err(err) => {
                if !errors::is_constraint_err(&err) {
                    return Err(err.into());
                }

                tx.query_row(
                    include_str!("../../../sql/modpack_versions/id_from_unique.sql"),
                    params![self.modpack_id, self.game_version],
                    |row| row.get(0),
                )?
            }
        };

        tx.commit()?;

        Ok(ModpackVersion {
            id: Some(id),
            modpack_id: self.modpack_id,
            game_version: self.game_version,
            installed: self.installed,
            loaded: self.loaded,
            state: PhantomData::<Saved>,
        })
    }
}

impl ModpackVersion {
    pub fn load(
        modpack_version_id: i64,
        app_handle: &tauri::AppHandle,
        db: &mut Connection,
    ) -> Result<(), Box<dyn Error>> {
        let load_modpack_version = include_str!("../../../sql/modpack_versions/load.sql");

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
        let mut mods_dir = files::get_mc_path();
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
            include_str!("../../../sql/modpack_versions/unload_all.sql");

        db.execute(unload_all_modpack_versions, [])?;

        // Get the minecraft mods directory
        let mut mods_dir = files::get_mc_path();
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

    pub fn from_id(
        db: &PooledConnection<SqliteConnectionManager>,
        id: i64,
    ) -> Result<ModpackVersion<Saved>, Box<dyn Error>> {
        let get_modpack_version = include_str!("../../../sql/modpack_versions/from_id.sql");

        let mut stmt = db.prepare(get_modpack_version)?;
        let modpack_version = stmt.query_row(params![id], |row| {
            Ok(ModpackVersion {
                id: Some(row.get(0)?),
                modpack_id: row.get(1)?,
                game_version: row.get(2)?,
                installed: row.get(3)?,
                loaded: row.get(4)?,
                state: PhantomData::<Saved>,
            })
        })?;

        Ok(modpack_version)
    }
}

impl ModpackVersion<Saved> {
    pub fn get_mod_versions(
        &self,
        db: &Connection,
    ) -> Result<Vec<ModVersion<Saved>>, Box<dyn Error>> {
        let get_mod_versions =
            include_str!("../../../sql/mod_versions/from_modpack_version_id.sql");

        let mut stmt = db.prepare(get_mod_versions)?;
        let rows = stmt.query_map(params![self.id], |row| {
            Ok(ModVersion {
                id: Some(row.get(0)?),
                mod_id: row.get(1)?,
                version_id: row.get(2)?,
                game_version: row.get(3)?,
                download_url: row.get(4)?,
                dependency_of: row.get(5)?,
                state: PhantomData::<Saved>,
            })
        })?;

        let mut mod_versions = Vec::new();

        for row in rows {
            mod_versions.push(row?);
        }

        Ok(mod_versions)
    }

    pub fn delete(
        self,
        app_handle: tauri::AppHandle,
        db: &mut Connection,
    ) -> Result<(), Box<dyn Error>> {
        let delete_modpack_version = include_str!("../../../sql/modpack_versions/delete.sql");

        let tx = db.transaction()?;

        if self.loaded {
            ModpackVersion::unload_all(&tx)?;
        }

        tx.execute(delete_modpack_version, params![self.id])?;

        // Remove the modpack version directory
        self.uninstall(&app_handle)?;

        tx.commit()?;

        Ok(())
    }
}
