use crate::{
    database::{
        self,
        joins::{ModJoin, ModpackJoin},
        models::{Mod, Modpack, ModpackVersion, NotSaved, Saved, Settings},
    },
    requests::search,
    DbState, ReqState,
};

#[tauri::command]
pub fn get_modpack(id: i64, db: tauri::State<'_, DbState>) -> Result<Modpack<Saved>, String> {
    let db = database::get_conn(db);
    Modpack::get(&db, id).map_err(|e| e.to_string())
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

    if Settings::get_stable_only(&db).map_err(|e| e.to_string())? {
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
pub fn load_modpack_version(
    id: i64,
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let mut db = database::get_conn(db);
    ModpackVersion::load(id, &app_handle, &mut db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn unload_modpack_versions(db: tauri::State<'_, DbState>) -> Result<(), String> {
    let db = database::get_conn(db);
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
