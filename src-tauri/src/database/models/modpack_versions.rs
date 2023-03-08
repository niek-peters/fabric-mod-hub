use derive_new::new;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::error::Error;

use super::DbModel;

#[derive(new, Serialize)]
pub struct ModpackVersion {
    #[new(default)]
    pub id: Option<i64>,
    pub modpack_id: i64,
    pub game_version: String,
    #[new(default)]
    pub installed: bool,
    #[new(default)]
    pub loaded: bool,
}

impl DbModel for ModpackVersion {
    fn save(&mut self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_modpack_version = include_str!("../../../sql/modpack_versions/create.sql");

        let tx = db.unchecked_transaction()?;

        tx.execute(
            create_modpack_version,
            params![self.modpack_id, self.game_version],
        )?;

        let id = tx.last_insert_rowid();

        tx.commit()?;

        self.id = Some(id);

        Ok(())
    }
}
