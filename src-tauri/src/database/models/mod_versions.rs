use derive_new::new;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::{error::Error, marker::PhantomData};

use crate::database::errors;

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

impl ModVersion<NotSaved> {
    pub fn save(self, db: &mut Connection) -> Result<ModVersion<Saved>, Box<dyn Error>> {
        let tx = db.transaction()?;

        let id = match tx.execute(
            include_str!("../../../sql/mod_versions/create.sql"),
            params![
                self.mod_id,
                self.version_id,
                self.game_version,
                self.download_url
            ],
        ) {
            Ok(_) => tx.last_insert_rowid(),
            Err(err) => {
                if !errors::is_constraint_err(&err) {
                    return Err(err.into());
                }

                tx.query_row(
                    include_str!("../../../sql/mod_versions/id_from_version_id.sql"),
                    params![self.version_id],
                    |row| row.get(0),
                )?
            }
        };

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
