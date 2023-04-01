use crate::{
    database::{
        self,
        joins::{ModJoin, ModpackJoin},
        models::{Mod, Modpack, ModpackVersion, NotSaved, Saved},
    },
    requests::search,
    DbState, ReqState,
};

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

    modpack
        .get_game_versions(&client.0)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_modpack(
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
pub fn get_mod_joins(id: i64, db: tauri::State<'_, DbState>) -> Vec<ModJoin> {
    let db = database::get_conn(db);
    ModJoin::get_by_modpack_version_id(&db, id)
        .expect(format!("Should get all mod joins related to modpack with id: {id}").as_str())
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
pub async fn search(
    query: String,
    client: tauri::State<'_, ReqState>,
) -> Result<Vec<Mod<NotSaved>>, String> {
    search::run(&client.0, query)
        .await
        .map_err(|e| e.to_string())
}
