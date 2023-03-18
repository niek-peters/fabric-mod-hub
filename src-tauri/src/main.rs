#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use database::{
    joins::{ModJoin, ModpackJoin},
    models::{Mod, Modpack, ModpackVersion, NotSaved, Saved, Settings},
    Database,
};
use dotenvy::dotenv;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use requests::search;
use reqwest::Client;
use std::fs::File;
use std::{
    error::Error,
    io::{self, Cursor},
};
use tauri::Manager;
use uuid::Uuid;

mod database;
mod files;
mod requests;

pub struct DbState(Database);
pub struct ReqState(Client);

const USER_AGENT: &str = "MrValk/fabric-mod-hub/0.0.1 (n.s.peters05@gmail.com)";

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(DbState(Database::init(&app.handle())));
            app.manage(ReqState(
                Client::builder()
                    .user_agent(USER_AGENT)
                    .build()
                    .expect("Should build reqwest client"),
            ));

            // Initialize settings
            let client = &app.state::<ReqState>().0;
            let mut db = database::get_conn(app.state::<DbState>());

            Settings::new("lol".to_string())
                .save(&db)
                .expect("Should initialize settings");

            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();

            main_window.hide().unwrap();

            let client_clone = client.clone();
            let app_handle = app.handle().clone();

            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // Initialize default modpacks
                create_default_modpack_versions(&app_handle, &client_clone, &mut db).await;

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_all_modpacks,
            get_all_modpack_joins,
            get_modpack_game_versions,
            install_modpack,
            get_mod_joins,
            load_modpack_version,
            unload_modpack_versions,
            uninstall_modpack_version,
            search
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_all_modpacks(db: tauri::State<'_, DbState>) -> Vec<Modpack<Saved>> {
    let db = database::get_conn(db);
    Modpack::get_all(&db)
}

#[tauri::command]
fn get_all_modpack_joins(db: tauri::State<'_, DbState>) -> Vec<ModpackJoin> {
    let db = database::get_conn(db);
    ModpackJoin::get_all(&db)
}

#[tauri::command]
async fn get_modpack_game_versions(
    id: i64,
    client: tauri::State<'_, ReqState>,
    db: tauri::State<'_, DbState>,
) -> Result<Vec<String>, String> {
    let db = database::get_conn(db);

    let modpack = Modpack::from_id(&db, id)
        .expect(format!("Should get the modpack related with id: {id}").as_str());

    modpack
        .get_game_versions(&client.0)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn install_modpack(
    id: i64,
    game_version: &str,
    app_handle: tauri::AppHandle,
    client: tauri::State<'_, ReqState>,
    db: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let client = &client.0;
    let mut db = database::get_conn(db);

    let modpack = Modpack::from_id(&db, id)
        .expect(format!("Should get the modpack related with id: {id}").as_str());

    let modpack_version = modpack
        .create_version(client, &mut db, game_version)
        .await
        .expect("Should create modpack version");

    modpack_version
        .install(&app_handle, &mut db, client)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_mod_joins(id: i64, db: tauri::State<'_, DbState>) -> Vec<ModJoin> {
    let db = database::get_conn(db);
    ModJoin::from_modpack_version_id(&db, id)
        .expect(format!("Should get all mod joins related to modpack with id: {id}").as_str())
}

#[tauri::command]
fn load_modpack_version(
    id: i64,
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let mut db = database::get_conn(db);
    ModpackVersion::load(id, &app_handle, &mut db).map_err(|e| e.to_string())
}

#[tauri::command]
fn unload_modpack_versions(db: tauri::State<'_, DbState>) -> Result<(), String> {
    let db = database::get_conn(db);
    ModpackVersion::unload_all(&db).map_err(|e| e.to_string())
}

#[tauri::command]
fn uninstall_modpack_version(id: i64, app_handle: tauri::AppHandle, db: tauri::State<'_, DbState>) {
    let mut db = database::get_conn(db);

    let modpack_version = ModpackVersion::from_id(&db, id)
        .expect(format!("Should get the modpack version related with id: {id}").as_str());

    modpack_version
        .delete(app_handle, &mut db)
        .expect("Should delete modpack version");
}

#[tauri::command]
async fn search(
    query: String,
    client: tauri::State<'_, ReqState>,
) -> Result<Vec<Mod<NotSaved>>, String> {
    search::run(&client.0, query)
        .await
        .map_err(|e| e.to_string())
}

async fn create_default_modpack_versions(
    app_handle: &tauri::AppHandle,
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
    let modpack_version1 = modpack1.create_version(client, db, "1.19.2").await.unwrap();
    modpack_version1
        .install(app_handle, db, client)
        .await
        .unwrap();

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
    let modpack_version2 = modpack2.create_version(client, db, "1.19.2").await.unwrap();
    modpack_version2
        .install(app_handle, db, client)
        .await
        .unwrap();

    let modpack3 = Modpack::create(
        client,
        db,
        "Death".to_string(),
        "death".to_string(),
        true,
        vec!["ssUbhMkL".to_string()],
    )
    .await
    .unwrap();
    let modpack_version3 = modpack3.create_version(client, db, "1.19.3").await.unwrap();
    modpack_version3
        .install(app_handle, db, client)
        .await
        .unwrap();

    println!("Minecraft dir: {}", files::get_mc_path().display())
}
