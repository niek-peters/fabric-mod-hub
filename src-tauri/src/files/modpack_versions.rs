use std::{error::Error, fs};

use reqwest::Client;
use rusqlite::Connection;

use crate::database::models::{ModVersion, ModpackVersion, Saved};

impl ModpackVersion<Saved> {
    pub async fn download(
        &self,
        app_handle: &tauri::AppHandle,
        db: &mut Connection,
        client: &Client,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.id.expect("Saved ModpackVersion should have an id");

        let mod_versions = self.get_mod_versions(db)?;

        let mut incl_dependencies: Vec<ModVersion<Saved>> = Vec::new();
        for mod_version in mod_versions {
            incl_dependencies.append(&mut mod_version.get_dependencies(db)?);
        }

        let mut path = super::get_data_path(app_handle);
        let dir_name = format!("{}/", id);
        path.push(dir_name);

        fs::create_dir_all(&path)
            .expect(format!("Should create ModpackVersion directory: {:?}", &path).as_str());

        for mod_version in incl_dependencies {
            mod_version.download(&mut path, client).await?;
        }

        Ok(())
    }
}
