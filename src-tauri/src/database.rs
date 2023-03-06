pub mod models;

use self::models::Modpack;

use include_sqlite_sql::{impl_sql, include_sql};
use rusqlite::Connection;
use std::{env, fs, path::PathBuf};

include_sql!("sql/init.sql");

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

        Self::create_tables(&connection);

        Self { connection, path }
    }

    fn create_tables(db: &Connection) {
        db.create_table_modpacks(|_| Ok(()))
            .expect("Could not create modpacks table");

        db.create_table_modpack_versions(|_| Ok(()))
            .expect("Could not create modpack_versions table");

        db.create_table_modpack_mods(|_| Ok(()))
            .expect("Could not create modpack_mods table");

        db.create_table_mods(|_| Ok(()))
            .expect("Could not create mods table");

        db.create_table_mod_versions(|_| Ok(()))
            .expect("Could not create mod_versions table");

        db.create_table_settings(|_| Ok(()))
            .expect("Could not create settings table");
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
