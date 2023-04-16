use std::{fs, path::PathBuf};

use crate::{
    database::{
        self,
        joins::{ModJoin, ModpackJoin},
        models::{Mod, Modpack, ModpackMod, ModpackVersion, NotSaved, Saved, Settings},
    },
    files::{self, fabric_loader, launcher_profile},
    requests::search,
    DbState, ReqState,
};

#[tauri::command]
pub fn get_modpack(id: i64, db: tauri::State<'_, DbState>) -> Result<Modpack<Saved>, String> {
    let db = database::get_conn(db);
    Modpack::get(&db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_modpack_mods(id: i64, db: tauri::State<'_, DbState>) -> Result<Vec<Mod<Saved>>, String> {
    let db = database::get_conn(db);
    Mod::get_by_modpack_id(id, &db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_modpacks(db: tauri::State<'_, DbState>) -> Vec<Modpack<Saved>> {
    let db = database::get_conn(db);
    Modpack::get_all(&db)
}

#[tauri::command]
pub fn get_all_modpack_joins(db: tauri::State<'_, DbState>) -> Vec<ModpackJoin> {
    let db = database::get_conn(db);
    ModpackJoin::get_all(&db)
}

#[tauri::command]
pub async fn get_modpack_game_versions(
    id: i64,
    client: tauri::State<'_, ReqState>,
    db: tauri::State<'_, DbState>,
) -> Result<Vec<String>, String> {
    let db = database::get_conn(db);

    let modpack = Modpack::get(&db, id)
        .expect(format!("Should get the modpack related with id: {id}").as_str());

    let mut game_versions = modpack
        .get_game_versions(&client.0)
        .await
        .map_err(|e| e.to_string())?;

    if !Settings::get_allow_snapshots(&db).map_err(|e| e.to_string())? {
        game_versions.retain(|v| !v.contains("w") && !v.contains("pre") && !v.contains("rc"));
    }

    // Filter out versions that are already installed
    let installed_versions = ModpackVersion::get_by_modpack_id(&db, id)
        .expect(
            format!("Should get all modpack versions related to modpack with id: {id}").as_str(),
        )
        .into_iter()
        .map(|v| v.game_version)
        .collect::<Vec<String>>();
    game_versions.retain(|v| !installed_versions.contains(v));

    Ok(game_versions)
}

#[tauri::command]
pub fn check_slug_exists(slug: &str, db: tauri::State<'_, DbState>) -> Result<bool, String> {
    let db = database::get_conn(db);
    Modpack::slug_exists(&db, slug).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_modpack(
    name: String,
    slug: String,
    project_ids: Vec<String>,
    client: tauri::State<'_, ReqState>,
    db: tauri::State<'_, DbState>,
) -> Result<Modpack<Saved>, String> {
    let client = &client.0;
    let mut db = database::get_conn(db);

    Modpack::create(&client, &mut db, name, slug, false, project_ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_modpack(
    id: i64,
    name: String,
    slug: String,
    removed_mod_ids: Vec<i64>,
    new_project_ids: Vec<String>,
    client: tauri::State<'_, ReqState>,
    db: tauri::State<'_, DbState>,
) -> Result<Modpack<Saved>, String> {
    let client = &client.0;
    let mut db = database::get_conn(db);

    let modpack = Modpack::get(&db, id).map_err(|e| e.to_string())?;
    let updated_modpack = modpack
        .update(name, slug, &mut db)
        .map_err(|e| e.to_string())?;

    // Remove all the mods that are no longer in the modpack
    for mod_id in removed_mod_ids {
        let mod1 = Mod::get(mod_id, &db).map_err(|e| e.to_string())?;
        mod1.delete(&mut db).map_err(|e| e.to_string())?;
    }

    // Add all the new mods to the modpack
    for project_id in new_project_ids {
        let mod1 = Mod::get_by_project_id(&client, project_id)
            .await
            .map_err(|e| e.to_string())?;
        let mod1 = mod1.save(&mut db).map_err(|e| e.to_string())?;

        let modpack_mod = ModpackMod::new(id, mod1.id.unwrap());
        modpack_mod.save(&mut db).map_err(|e| e.to_string())?;
    }

    Ok(updated_modpack)
}

#[tauri::command]
pub fn delete_modpack(id: i64, db: tauri::State<'_, DbState>) -> Result<(), String> {
    let mut db = database::get_conn(db);

    let modpack = Modpack::get(&db, id).map_err(|e| e.to_string())?;
    modpack.delete(&mut db).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_mod_joins(id: i64, db: tauri::State<'_, DbState>) -> Vec<ModJoin> {
    let db = database::get_conn(db);
    ModJoin::get_by_modpack_version_id(&db, id)
        .expect(format!("Should get all mod joins related to modpack with id: {id}").as_str())
}

#[tauri::command]
pub async fn install_modpack_version(
    id: i64,
    game_version: &str,
    app_handle: tauri::AppHandle,
    client: tauri::State<'_, ReqState>,
    db: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let client = &client.0;
    let mut db = database::get_conn(db);

    let modpack = Modpack::get(&db, id)
        .expect(format!("Should get the modpack related with id: {id}").as_str());

    let modpack_version =
        ModpackVersion::create_from_modpack(modpack, client, &mut db, game_version)
            .await
            .expect("Should create modpack version");

    modpack_version
        .install(&app_handle, &mut db, client)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn uninstall_modpack_version(
    id: i64,
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, DbState>,
) {
    let mut db = database::get_conn(db);

    let modpack_version = ModpackVersion::get(&db, id)
        .expect(format!("Should get the modpack version related with id: {id}").as_str());

    modpack_version
        .delete(app_handle, &mut db)
        .expect("Should delete modpack version");
}

#[tauri::command]
pub async fn load_modpack_version(
    id: i64,
    app_handle: tauri::AppHandle,
    client: tauri::State<'_, ReqState>,
    db: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let client = &client.0;
    let mut db = database::get_conn(db);
    let mc_path = Settings::get_minecraft_dir(&db).map_err(|e| e.to_string())?;

    let game_version = ModpackVersion::get(&db, id)
        .expect(format!("Should get the modpack version related with id: {id}").as_str())
        .game_version;

    // Create the version folder and its contents
    let mut version_path = mc_path.clone();
    version_path.push("versions");
    version_path.push(format!("FabricModHub-{}", game_version));

    fabric_loader::download(&version_path, game_version.clone(), client)
        .await
        .map_err(|e| e.to_string())?;

    // Add the launcher profile
    let mut launcher_profiles_path = mc_path;
    launcher_profiles_path.push("launcher_profiles.json");

    launcher_profile::add_profile(&launcher_profiles_path, game_version)
        .map_err(|e| e.to_string())?;

    // Load the mod files for the modpack version
    ModpackVersion::load(id, &app_handle, &mut db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn unload_modpack_versions(db: tauri::State<'_, DbState>) -> Result<(), String> {
    let db = database::get_conn(db);
    let mc_path = Settings::get_minecraft_dir(&db).map_err(|e| e.to_string())?;

    // Remove all version folders containing "FabricModHub"
    let mut version_path = mc_path.clone();
    version_path.push("versions");
    let paths = fs::read_dir(&version_path).map_err(|e| e.to_string())?;
    for path in paths {
        let path = path.map_err(|e| e.to_string())?;
        let path = path.path();
        if path.is_dir() {
            let path = path.to_str().unwrap();
            if path.contains("FabricModHub") {
                fs::remove_dir_all(path).map_err(|e| e.to_string())?;
            }
        }
    }

    // Remove all launcher profiles containing "FabricModHub"
    let mut launcher_profiles_path = mc_path;
    launcher_profiles_path.push("launcher_profiles.json");
    launcher_profile::remove_profiles(&launcher_profiles_path).map_err(|e| e.to_string())?;

    // Unload all mod files for the modpack versions
    ModpackVersion::unload_all(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search(
    query: String,
    client: tauri::State<'_, ReqState>,
) -> Result<Vec<Mod<NotSaved>>, String> {
    search::run(&client.0, query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn init_settings(db: tauri::State<'_, DbState>) -> Result<(), String> {
    let db = database::get_conn(db);

    let mut failed = false;

    // Initialize settings
    let mc_dir = match files::get_default_minecraft_path() {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(_) => {
            failed = true;
            "".to_string()
        }
    };

    Settings::new(mc_dir)
        .save(&db)
        .expect("Should initialize settings");

    if failed {
        Err("Failed to get Minecraft path".to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn get_settings(db: tauri::State<'_, DbState>) -> Result<Settings<Saved>, String> {
    let db = database::get_conn(db);
    Settings::get(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_allow_unstable(
    allow_unstable: bool,
    db: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let mut db = database::get_conn(db);
    Settings::set_allow_unstable(&mut db, allow_unstable).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_allow_snapshots(
    allow_snapshots: bool,
    db: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let mut db = database::get_conn(db);
    Settings::set_allow_snapshots(&mut db, allow_snapshots).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_minecraft_dir(
    minecraft_dir: String,
    db: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let mut db = database::get_conn(db);

    let minecraft_dir = PathBuf::from(minecraft_dir);
    match files::is_minecraft_dir(&minecraft_dir) {
        true => Settings::set_minecraft_dir(&mut db, minecraft_dir).map_err(|e| e.to_string()),
        false => Err("Not a valid Minecraft directory".to_string()),
    }
}
