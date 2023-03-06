pub mod models;

use self::models::*;

use include_sqlite_sql::{impl_sql, include_sql};
use rusqlite::Connection;
use std::{env, fs, path::PathBuf};

include_sql!("sql/init.sql");
include_sql!("sql/settings.sql");

pub struct Database {
    pub connection: Connection,
    pub path: PathBuf,
}

impl Database {
    pub fn init(app_handle: &tauri::AppHandle) -> Self {
        let path = get_db_path(app_handle);
        let connection = Connection::open(&path).expect(
            format!(
                "Should be able to open database with supplied path: {:?}",
                &path
            )
            .as_str(),
        );

        Self::init_tables(&connection);

        Self { connection, path }
    }

    fn init_tables(db: &Connection) {
        db.create_tables()
            .expect("Should create all tables that don't exist");

        db.default_settings("blyat")
            .expect("Should create settings table");
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

        path.push("db.sqlite3");
    }

    path
}

fn is_dev() -> bool {
    env::var("TAURI_ENV").unwrap_or_else(|_| "prod".to_string()) == "dev"
}
