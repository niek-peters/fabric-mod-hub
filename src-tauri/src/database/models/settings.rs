use rusqlite::{params, Connection};
use std::error::Error;

use super::DbModel;

pub struct Settings {
    pub id: i64,
    pub minecraft_dir: String,
    pub stable_only: bool,
}

impl DbModel for Settings {
    fn save(&self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_settings = include_str!("../../../sql/settings/create.sql");

        db.execute(create_settings, params![self.minecraft_dir])?;

        Ok(())
    }
}

impl Settings {
    pub fn new(minecraft_dir: String, stable_only: bool) -> Self {
        Settings {
            id: 1,
            minecraft_dir,
            stable_only,
        }
    }
}
