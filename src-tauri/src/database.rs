pub mod errors;
pub mod joins;
pub mod models;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use std::{path::PathBuf, sync::Arc};

use crate::{files, DbState};

pub struct Database(pub Arc<Pool<SqliteConnectionManager>>);

impl Database {
    pub fn init(app_handle: &tauri::AppHandle) -> Self {
        let path = get_db_path(app_handle);

        let man = SqliteConnectionManager::file(path.clone());
        let pool = r2d2::Pool::new(man).expect(
            format!(
                "Should open connection pool for database with supplied path: {:?}",
                &path
            )
            .as_str(),
        );
        let pool_arc = Arc::new(pool);

        let db = pool_arc
            .clone()
            .get()
            .expect(format!("Should get connection pool").as_str());

        let foreign_key_constraints = include_str!("../sql/foreign_key_constraints.sql");
        db.execute(foreign_key_constraints, [])
            .expect("Should enable foreign key constraints");

        let create_tables = include_str!("../sql/init.sql");
        db.execute_batch(create_tables)
            .expect("Should create all tables that don't exist");

        Self(pool_arc)
    }
}

fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let mut path = files::get_data_path(app_handle);

    if files::is_dev() {
        path.push("dev.sqlite3");
    } else {
        path.push("db.sqlite3");
    }

    path
}

pub fn get_conn(db_state: tauri::State<'_, DbState>) -> PooledConnection<SqliteConnectionManager> {
    db_state
        .0
         .0
        .clone()
        .get()
        .expect(format!("Should get connection pool").as_str())
}
