use rusqlite::{params, Connection};
use std::error::Error;

use super::DbModel;

pub struct ModpackVersion {
    pub id: Option<i64>,
    pub modpack_id: i64,
    pub game_version: String,
    pub installed: bool,
    pub loaded: bool,
}

impl DbModel for ModpackVersion {
    fn save(&self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_modpack_version = include_str!("../../../sql/modpack_versions/create.sql");

        db.execute(
            create_modpack_version,
            params![self.modpack_id, self.game_version],
        )?;

        Ok(())
    }
}

impl ModpackVersion {
    pub fn new(modpack_id: i64, game_version: String) -> Self {
        ModpackVersion {
            id: None,
            modpack_id,
            game_version,
            installed: false,
            loaded: false,
        }
    }
}
