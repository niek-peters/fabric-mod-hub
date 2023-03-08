use derive_new::new;
use rusqlite::Connection;
use std::error::Error;

use super::DbModel;

#[derive(new)]
pub struct Settings {
    #[new(value = "0")]
    pub id: i64,
    pub minecraft_dir: String,
    #[new(value = "true")]
    pub stable_only: bool,
}

impl DbModel for Settings {
    fn save(&mut self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_settings = include_str!("../../../sql/settings/create.sql");

        db.execute(create_settings, [&self.minecraft_dir])?;

        self.id = 0;

        Ok(())
    }
}
