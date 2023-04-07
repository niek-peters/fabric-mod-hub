#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use database::{
    models::{Modpack, Settings},
    Database,
};
use dotenvy::dotenv;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::Client;
use tauri::Manager;

mod commands;
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

            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            main_window.hide().unwrap();

            let client = (&app.state::<ReqState>().0).clone();
            let mut db = database::get_conn(app.state::<DbState>());

            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // Initialize settings
                let mc_dir = files::get_mc_path()
                    .to_str()
                    .expect("Should get mc path")
                    .to_owned();

                Settings::new(mc_dir)
                    .save(&db)
                    .expect("Should initialize settings");

                // Initialize default modpacks
                create_default_modpacks(&client, &mut db).await;

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_modpack,
            commands::get_all_modpacks,
            commands::get_all_modpack_joins,
            commands::get_modpack_game_versions,
            commands::check_slug_exists,
            commands::add_modpack,
            commands::delete_modpack,
            commands::get_mod_joins,
            commands::install_modpack_version,
            commands::uninstall_modpack_version,
            commands::load_modpack_version,
            commands::unload_modpack_versions,
            commands::search
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn create_default_modpacks(
    client: &Client,
    db: &mut PooledConnection<SqliteConnectionManager>,
) {
    Modpack::create(
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

    Modpack::create(
        client,
        db,
        "Multiplayer".to_string(),
        "multiplayer".to_string(),
        true,
        vec!["9eGKb6K1".to_string()],
    )
    .await
    .unwrap();

    Modpack::create(
        client,
        db,
        "Death".to_string(),
        "death".to_string(),
        true,
        vec!["ssUbhMkL".to_string()],
    )
    .await
    .unwrap();
}
