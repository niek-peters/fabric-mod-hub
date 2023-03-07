#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use dotenv::dotenv;
use reqwest::Client;
use tauri::Manager;

mod database;
use database::{
    models::{Mod, ModVersion},
    Database,
};

mod requests;

pub struct DbState(Mutex<Database>);
pub struct ReqState(Client);

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(DbState(Mutex::new(Database::init(&app.handle()))));
            app.manage(ReqState(Client::new()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![test_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn test_request(client: tauri::State<'_, ReqState>) -> Result<ModVersion, ()> {
    Ok(Mod::new(
        "P7dR8mSH".to_string(),
        "Gravestones".to_string(),
        "gravestones".to_string(),
        "blyat lol".to_string(),
    )
    .get_version(&client.inner().0, "1.19.2")
    .await
    .expect("Could not get version"))
}
