use derive_new::new;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::error::Error;

use super::DbModel;

#[derive(new, Serialize)]
pub struct ModVersion {
    #[new(default)]
    pub id: Option<i64>,
    pub mod_id: i64,
    pub version_id: String,
    pub game_version: String,
    pub download_url: String,
    pub dependencies: Vec<String>,
}

impl DbModel for ModVersion {
    fn save(&mut self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_mod_version = include_str!("../../../sql/mod_versions/create.sql");

        let tx = db.unchecked_transaction()?;

        tx.execute(
            create_mod_version,
            params![
                self.mod_id,
                self.version_id,
                self.game_version,
                self.download_url
            ],
        )?;

        let id = tx.last_insert_rowid();

        tx.commit()?;

        self.id = Some(id);

        Ok(())
    }
}
