use rusqlite::Connection;
use std::{error::Error, marker::PhantomData};

use crate::database::models::{NotSaved, Saved, Settings};

impl Settings<NotSaved> {
    pub fn get(db: &Connection) -> Result<Settings<Saved>, Box<dyn Error>> {
        let get_settings = include_str!("../../../../sql/settings/get.sql");

        let mut stmt = db.prepare(get_settings)?;
        let settings = stmt.query_row([], |row| {
            Ok(Settings {
                id: row.get(0)?,
                minecraft_dir: row.get(1)?,
                allow_unstable: row.get(2)?,
                allow_snapshots: row.get(3)?,
                state: PhantomData::<Saved>,
            })
        })?;

        Ok(settings)
    }

    pub fn save(self, db: &Connection) -> Result<Settings<Saved>, Box<dyn Error>> {
        let create_settings = include_str!("../../../../sql/settings/create.sql");

        db.execute(create_settings, [&self.minecraft_dir])?;

        Ok(Settings {
            id: 0,
            minecraft_dir: self.minecraft_dir,
            allow_unstable: self.allow_unstable,
            allow_snapshots: self.allow_snapshots,
            state: PhantomData::<Saved>,
        })
    }
}
