#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;

use dotenvy::dotenv;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::Client;
use rusqlite::Connection;
use tauri::Manager;

mod database;
use database::{
    joins::{ModJoin, ModpackJoin},
    models::{Modpack, ModpackVersion, Saved, Settings},
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
            let client = &app.state::<ReqState>().0;
            let mut db = database::get_conn(app.state::<DbState>());

            Settings::new("lol".to_string())
                .save(&db)
                .expect("Should initialize settings");

            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();

            let client_clone = client.clone();

            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // Initialize default modpacks
                create_default_modpack_versions(&client_clone, &mut db).await;

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
                main_window
                    .emit("loaded", get_all_modpack_joins(&mut db))
                    .unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            test_request,
            get_all_modpacks,
            get_mod_joins_from_modpack_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_all_modpack_joins(db: &mut Connection) -> Vec<ModpackJoin> {
    ModpackJoin::get_all(db)
}

#[tauri::command]
fn get_all_modpacks(db: tauri::State<'_, DbState>) -> Vec<Modpack<Saved>> {
    let db = database::get_conn(db);
    Modpack::get_all(&db)
}

#[tauri::command]
fn get_mod_joins_from_modpack_id(id: i64, db: tauri::State<'_, DbState>) -> Vec<ModJoin> {
    let db = database::get_conn(db);
    ModJoin::get_from_modpack_version_id(&db, id)
        .expect(format!("Should get all mod joins related to modpack with id: {id}").as_str())
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

    modpack.create_version(client, db, "1.19.2").await
}

async fn create_default_modpack_versions(
    client: &Client,
    db: &mut PooledConnection<SqliteConnectionManager>,
) {
    let modpack1 = Modpack::create(
        client,
        db,
        "Optimizations".to_string(),
        "optimizations".to_string(),
        true,
        vec![
            "gvQqBUqZ".to_string(),
            "AANobbMI".to_string(),
            "H8CaAYZC".to_string(),
        ],
    )
    .await
    .unwrap();

    modpack1.create_version(client, db, "1.19.2").await.unwrap();

    let modpack2 = Modpack::create(
        client,
        db,
        "Multiplayer".to_string(),
        "multiplayer".to_string(),
        true,
        vec!["9eGKb6K1".to_string()],
    )
    .await
    .unwrap();

    modpack2.create_version(client, db, "1.19.2").await.unwrap();
}
