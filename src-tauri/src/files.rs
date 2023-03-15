pub mod mod_versions;
pub mod modpack_versions;

use std::{
    env::{self, consts::OS},
    fs,
    path::PathBuf,
};

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

pub fn get_mc_path() -> PathBuf {
    if is_dev() {
        let mut dir = PathBuf::from("data/.minecraft/");

        dir.push("mods");

        fs::create_dir_all(&dir)
            .expect(format!("Should create minecraft mods directory: {:?}", &dir).as_str());

        dir.pop();

        dir.push("versions");

        fs::create_dir_all(&dir)
            .expect(format!("Should create minecraft versions directory: {:?}", &dir).as_str());

        dir.pop();

        return dir;
    }

    match OS {
        "windows" => {
            let mut dir = dirs::config_dir().expect("Should get %APPDATA% dir");
            dir.push(".minecraft");
            dir
        }
        "macos" => {
            let mut dir = dirs::config_dir().expect("Should get Application Support dir");
            dir.push("minecraft");
            dir
        }
        "linux" => {
            let mut dir = dirs::home_dir().expect("Should get home dir");
            dir.push(".minecraft");
            dir
        }
        _ => panic!("Unsupported OS"),
    }
}

pub fn is_dev() -> bool {
    env::var("TAURI_ENV").unwrap_or_else(|_| "prod".to_string()) == "dev"
}
