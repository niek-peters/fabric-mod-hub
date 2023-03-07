pub mod models;

use rusqlite::Connection;
use std::{env, fs, path::PathBuf};

pub struct Database(Connection);

impl Database {
    pub fn init(app_handle: &tauri::AppHandle) -> Self {
        let path = get_db_path(app_handle);
        let db = Connection::open(&path).expect(
            format!(
                "Should be able to open database with supplied path: {:?}",
                &path
            )
            .as_str(),
        );

        let foreign_key_constraints = include_str!("../sql/foreign_key_constraints.sql");
        db.execute(foreign_key_constraints, [])
            .expect("Should enable foreign key constraints");

        let create_tables = include_str!("../sql/init.sql");
        db.execute_batch(create_tables)
            .expect("Should create all tables that don't exist");

        Self(db)
    }
}

fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let mut path = PathBuf::new();

    if is_dev() {
        path.push("dev.sqlite3");
    } else {
        let data_config_dir = app_handle
            .path_resolver()
            .app_config_dir()
            .expect("Should be able to get app config directory");
        fs::create_dir_all(&data_config_dir).expect(
            format!(
                "Should be able to create app config directory: {:?}",
                &data_config_dir
            )
            .as_str(),
        );

        path.push(data_config_dir);
        path.push("db.sqlite3");
    }

    path
}

fn is_dev() -> bool {
    env::var("TAURI_ENV").unwrap_or_else(|_| "prod".to_string()) == "dev"
}
