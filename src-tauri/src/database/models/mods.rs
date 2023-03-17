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

impl Mod {
    pub fn from_modpack_id(
        modpack_id: i64,
        db: &Connection,
    ) -> Result<Vec<Mod<Saved>>, Box<dyn Error>> {
        let mut stmt = db.prepare(include_str!("../../../sql/mods/get_from_modpack_id.sql"))?;
        let mods_iter = stmt.query_map(params![modpack_id], |row| {
            Ok(Mod {
                id: Some(row.get(0)?),
                project_id: row.get(1)?,
                name: row.get(2)?,
                slug: row.get(3)?,
                page_url: row.get(4)?,
                state: PhantomData::<Saved>,
            })
        })?;

        let mut mods: Vec<Mod<Saved>> = Vec::new();
        for mod1 in mods_iter {
            mods.push(mod1?);
        }

        Ok(mods)
    }
}