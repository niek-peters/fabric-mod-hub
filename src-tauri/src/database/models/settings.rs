use derive_new::new;
use rusqlite::{params, Connection};
use std::error::Error;

use super::DbModel;

#[derive(new)]
pub struct Settings {
    #[new(value = "1")]
    pub id: i64,
    pub minecraft_dir: String,
    #[new(value = "true")]
    pub stable_only: bool,
}

impl DbModel for Settings {
    fn save(&self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_settings = include_str!("../../../sql/settings/create.sql");

        db.execute(create_settings, params![self.minecraft_dir])?;

        Ok(())
    }
}
