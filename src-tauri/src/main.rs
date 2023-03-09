#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;

use dotenv::dotenv;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::Client;
use tauri::Manager;

mod database;
use database::{
    models::{Mod, Modpack, ModpackVersion, Saved, Settings},
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

            // Initialize settings
            let db = database::get_conn(app.state::<DbState>());
            Settings::new("lol".to_string())
                .save(&db)
                .expect("Should initialize settings");

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
) -> Result<ModpackVersion<Saved>, String> {
    let client = &client.0;
    let mut db = database::get_conn(db);

    test(client, &mut db).await.map_err(|e| e.to_string())
}

async fn test(
    client: &Client,
    db: &mut PooledConnection<SqliteConnectionManager>,
) -> Result<ModpackVersion<Saved>, Box<dyn Error>> {
    let modpack = Modpack::create(
        client,
        db,
        "Test2".to_string(),
        "test2".to_string(),
        true,
        vec!["ssUbhMkL".to_string()],
    )
    .await?;

    // Get all ModVersions from modpack's list of mods
    for mod1 in &modpack.mods {
        mod1.get_version(client, "1.19.2").await?.save(db)?;
    }

    let modpack_version = modpack.get_version("1.19.2").save(db)?;

    Ok(modpack_version)
}
