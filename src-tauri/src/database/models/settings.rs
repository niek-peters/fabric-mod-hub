use derive_new::new;
use rusqlite::Connection;
use std::{error::Error, marker::PhantomData};

use super::{NotSaved, Saved};

#[derive(new)]
pub struct Settings<State = NotSaved> {
    #[new(value = "0")]
    pub id: i64,
    pub minecraft_dir: String,
    #[new(value = "true")]
    pub stable_only: bool,
    state: PhantomData<State>,
}

impl Settings<NotSaved> {
    pub fn save(self, db: &Connection) -> Result<Settings<Saved>, Box<dyn Error>> {
        let create_settings = include_str!("../../../sql/settings/create.sql");

        db.execute(create_settings, [&self.minecraft_dir])?;

        Ok(Settings {
            id: 0,
            minecraft_dir: self.minecraft_dir,
            stable_only: self.stable_only,
            state: PhantomData::<Saved>,
        })
    }
}
