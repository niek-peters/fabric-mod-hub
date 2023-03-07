use rusqlite::{params, Connection};
use std::error::Error;

use super::DbModel;

pub struct ModVersion {
    pub id: Option<i64>,
    pub mod_id: i64,
    pub version_id: String,
    pub game_version: String,
    pub download_url: String,
}

impl DbModel for ModVersion {
    fn save(&self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_mod_version = include_str!("../../../sql/mod_versions/create.sql");

        db.execute(
            create_mod_version,
            params![
                self.mod_id,
                self.version_id,
                self.game_version,
                self.download_url
            ],
        )?;

        Ok(())
    }
}

impl ModVersion {
    pub fn new(
        mod_id: i64,
        version_id: String,
        game_version: String,
        download_url: String,
    ) -> Self {
        ModVersion {
            id: None,
            mod_id,
            version_id,
            game_version,
            download_url,
        }
    }
}
