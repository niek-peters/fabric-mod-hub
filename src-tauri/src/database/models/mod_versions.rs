use derive_new::new;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::{error::Error, marker::PhantomData};

use super::{NotSaved, Saved};

#[derive(new, Serialize)]
pub struct ModVersion<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub mod_id: i64,
    pub version_id: String,
    pub game_version: String,
    pub download_url: String,
    pub dependencies: Vec<String>,
    state: PhantomData<State>,
}

impl ModVersion {
    pub fn save(self, db: &mut Connection) -> Result<ModVersion<Saved>, Box<dyn Error>> {
        let create_mod_version = include_str!("../../../sql/mod_versions/create.sql");

        let tx = db.transaction()?;

        tx.execute(
            create_mod_version,
            params![
                self.mod_id,
                self.version_id,
                self.game_version,
                self.download_url
            ],
        )?;

        let id = tx.last_insert_rowid();

        tx.commit()?;

        Ok(ModVersion {
            id: Some(id),
            mod_id: self.mod_id,
            version_id: self.version_id,
            game_version: self.game_version,
            download_url: self.download_url,
            dependencies: self.dependencies,
            state: PhantomData::<Saved>,
        })
    }
}
