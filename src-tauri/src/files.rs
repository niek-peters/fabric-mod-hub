pub mod mod_versions;
pub mod modpack_versions;

use std::{env, fs, path::PathBuf};

pub fn get_data_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let data_dir: PathBuf;

    if is_dev() {
        data_dir = PathBuf::from("data/");
    } else {
        data_dir = app_handle
            .path_resolver()
            .app_config_dir()
            .expect("Should get app config directory");
    }

    fs::create_dir_all(&data_dir)
        .expect(format!("Should create app config directory: {:?}", &data_dir).as_str());

    data_dir
}

pub fn is_dev() -> bool {
    env::var("TAURI_ENV").unwrap_or_else(|_| "prod".to_string()) == "dev"
}
