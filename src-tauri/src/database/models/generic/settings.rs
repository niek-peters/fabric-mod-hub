use rusqlite::Connection;
use std::{error::Error, marker::PhantomData};

use crate::database::models::{NotSaved, Saved, Settings};

impl Settings<NotSaved> {
    pub fn save(self, db: &Connection) -> Result<Settings<Saved>, Box<dyn Error>> {
        let create_settings = include_str!("../../../../sql/settings/create.sql");

        db.execute(create_settings, [&self.minecraft_dir])?;

        Ok(Settings {
            id: 0,
            minecraft_dir: self.minecraft_dir,
            stable_only: self.stable_only,
            state: PhantomData::<Saved>,
        })
    }
}
