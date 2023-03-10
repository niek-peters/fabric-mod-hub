use derive_new::new;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::{error::Error, marker::PhantomData};

use crate::database::errors;

use super::{NotSaved, Saved};

#[derive(new, Serialize)]
pub struct Mod<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub project_id: String,
    pub name: String,
    pub slug: String,
    pub page_url: String,
    state: PhantomData<State>,
}

impl Mod<NotSaved> {
    pub fn save(self, db: &mut Connection) -> Result<Mod<Saved>, Box<dyn Error>> {
        let tx = db.transaction()?;

        let id = match tx.execute(
            include_str!("../../../sql/mods/create.sql"),
            params![self.project_id, self.name, self.slug, self.page_url],
        ) {
            Ok(_) => tx.last_insert_rowid(),
            Err(err) => {
                if !errors::is_constraint_err(&err) {
                    return Err(err.into());
                }

                tx.query_row(
                    include_str!("../../../sql/mods/get_from_project_id.sql"),
                    params![self.project_id],
                    |row| row.get(0),
                )?
            }
        };

        tx.commit()?;

        Ok(Mod {
            id: Some(id),
            project_id: self.project_id,
            name: self.name,
            slug: self.slug,
            page_url: self.page_url,
            state: PhantomData::<Saved>,
        })
    }
}
