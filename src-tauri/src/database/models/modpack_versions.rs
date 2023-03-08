use derive_new::new;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::{error::Error, marker::PhantomData};

use super::{NotSaved, Saved};

#[derive(new, Serialize)]
pub struct ModpackVersion<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub modpack_id: i64,
    pub game_version: String,
    #[new(default)]
    pub installed: bool,
    #[new(default)]
    pub loaded: bool,
    state: PhantomData<State>,
}

impl ModpackVersion {
    pub fn save(self, db: &mut Connection) -> Result<ModpackVersion<Saved>, Box<dyn Error>> {
        let create_modpack_version = include_str!("../../../sql/modpack_versions/create.sql");

        let tx = db.transaction()?;

        tx.execute(
            create_modpack_version,
            params![self.modpack_id, self.game_version],
        )?;

        let id = tx.last_insert_rowid();

        tx.commit()?;

        Ok(ModpackVersion {
            id: Some(id),
            modpack_id: self.modpack_id,
            game_version: self.game_version,
            installed: self.installed,
            loaded: self.loaded,
            state: PhantomData::<Saved>,
        })
    }
}
