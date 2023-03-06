#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dotenv::dotenv;
use std::sync::Mutex;
use tauri::Manager;

mod database;
use database::Database;

pub struct DBState(pub Mutex<Database>);

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(DBState(Mutex::new(Database::init(&app.handle()))));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
