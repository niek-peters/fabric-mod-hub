pub mod fabric_loader;
pub mod launcher_profile;
pub mod mod_versions;
pub mod modpack_versions;

use std::{
    collections::HashMap,
    env::{self, consts::OS},
    fs,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

const MINECRAFT_FOLDER_CONTENTS: [&str; 6] = [
    "assets",
    "libraries",
    "logs",
    "versions",
    "options.txt",
    "launcher_profiles.json",
];

#[derive(Serialize, Deserialize)]
pub struct LauncherProfiles {
    pub profiles: HashMap<String, LauncherProfile>,
    pub settings: LauncherSettings,
    pub version: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherProfile {
    pub created: String,
    pub icon: String,
    pub last_used: String,
    pub last_version_id: String,
    pub name: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherSettings {
    pub crash_assistance: bool,
    pub enable_advanced: bool,
    pub enable_analytics: bool,
    pub enable_historical: bool,
    pub enable_releases: bool,
    pub enable_snapshots: bool,
    pub keep_launcher_open: bool,
    pub profile_sorting: String,
    pub show_game_log: bool,
    pub show_menu: bool,
    pub sound_on: bool,
}

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

pub fn get_default_minecraft_path() -> Result<PathBuf, String> {
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

        dir.push("launcher_profiles.json");

        if !dir.exists() {
            fs::write(
                &dir,
                serde_json::to_string_pretty(&LauncherProfiles {
                    profiles: HashMap::new(),
                    settings: LauncherSettings {
                        crash_assistance: true,
                        enable_advanced: false,
                        enable_analytics: true,
                        enable_historical: false,
                        enable_releases: true,
                        enable_snapshots: false,
                        keep_launcher_open: false,
                        profile_sorting: "ByLastPlayed".to_string(),
                        show_game_log: false,
                        show_menu: false,
                        sound_on: false,
                    },
                    version: 3,
                })
                .unwrap(),
            )
            .expect(format!("Should create launcher_profiles.json: {:?}", &dir).as_str());
        }

        dir.pop();

        return Ok(dir);
    }

    let mc_path = match OS {
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
    };

    if !is_minecraft_dir(&mc_path) {
        return Err("Directory is not a Minecraft directory".to_string());
    }

    Ok(mc_path)
}

pub fn is_minecraft_dir(dir: &PathBuf) -> bool {
    if !dir.is_dir() {
        return false;
    }

    for file in MINECRAFT_FOLDER_CONTENTS.iter() {
        if !dir.join(file).exists() {
            return false;
        }
    }

    true
}

pub fn is_dev() -> bool {
    env::var("TAURI_ENV").unwrap_or_else(|_| "prod".to_string()) == "dev"
}
