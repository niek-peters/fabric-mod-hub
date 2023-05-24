use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::database::models::{CustomFile, ModpackVersion, Saved};
use crate::files;

impl CustomFile<Saved> {
    pub fn add_to_modpack_version(
        &self,
        filepath: String,
        modpack_version_id: i64,
        app_handle: &tauri::AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        // Check if the version has been downloaded
        if !ModpackVersion::is_installed(modpack_version_id, app_handle) {
            return Err("Cannot add file to modpack version that hasn't been downloaded".into());
        }

        // Check if the file exists
        if !Path::new(&filepath).exists() {
            return Err("Custom file does not exist".into());
        }

        // Get the modpack version directory
        let mut version_dir = files::get_data_path(app_handle);
        let dir_name = format!("{}/", modpack_version_id);
        version_dir.push(dir_name);

        // Copy the file to the modpack version directory
        let mut new_path = version_dir.clone();
        new_path.push(self.filename.clone());
        fs::copy(filepath, new_path)?;

        Ok(())
    }

    pub fn remove_from_modpack_version(
        &self,
        modpack_version_id: i64,
        app_handle: &tauri::AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        let file_path = CustomFile::get_path(self, modpack_version_id, app_handle)?;

        // Delete the file
        fs::remove_file(file_path)?;

        Ok(())
    }

    pub fn get_path(
        &self,
        modpack_version_id: i64,
        app_handle: &tauri::AppHandle,
    ) -> Result<PathBuf, Box<dyn Error>> {
        // Check if the version has been downloaded
        if !ModpackVersion::is_installed(modpack_version_id, app_handle) {
            return Err(
                "Cannot get path of file from modpack version that hasn't been downloaded".into(),
            );
        }

        // Get the modpack version directory
        let mut version_dir = files::get_data_path(app_handle);
        let dir_name = format!("{}/", modpack_version_id);
        version_dir.push(dir_name);

        // Check if the file exists
        let mut file_path = version_dir.clone();
        file_path.push(self.filename.clone());
        if !file_path.exists() {
            return Err("Custom file does not exist".into());
        }

        Ok(file_path)
    }
}
