#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dotenv::dotenv;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::Client;
use tauri::Manager;

mod database;
use database::{
    models::{DbModel, Mod, Modpack, ModpackVersion},
    Database,
};

mod requests;

pub struct DbState(Database);
pub struct ReqState(Client);

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(DbState(Database::init(&app.handle())));
            app.manage(ReqState(Client::new()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![test_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn test_request(
    client: tauri::State<'_, ReqState>,
    db: tauri::State<'_, DbState>,
) -> Result<ModpackVersion, String> {
    let client = &client.0;
    let conn = get_conn(&db);

    let mut mod1 = Mod::from_project_id(client, "ssUbhMkL".to_string())
        .await
        .map_err(|e| e.to_string())?;
    mod1.save(&conn).map_err(|e| e.to_string())?;

    let mut modpack = Modpack::new("Test".to_string(), "test".to_string(), true, vec![mod1]);
    modpack.save(&conn).map_err(|e| e.to_string())?;

    let mut modpack_version = modpack
        .get_version(client, get_conn(&db), "1.19.2")
        .await
        .map_err(|e| e.to_string())?;
    modpack_version.save(&conn).map_err(|e| e.to_string())?;

    Ok(modpack_version)
}

fn get_conn(db: &tauri::State<'_, DbState>) -> PooledConnection<SqliteConnectionManager> {
    db.0 .0
        .clone()
        .get()
        .expect(format!("Should be able to get connection pool").as_str())
}
