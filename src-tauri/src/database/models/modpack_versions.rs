use derive_new::new;
use rusqlite::{params, Connection};
use std::error::Error;

use super::DbModel;

#[derive(new)]
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
    fn save(&self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_modpack_version = include_str!("../../../sql/modpack_versions/create.sql");

        db.execute(
            create_modpack_version,
            params![self.modpack_id, self.game_version],
        )?;

        Ok(())
    }
}
