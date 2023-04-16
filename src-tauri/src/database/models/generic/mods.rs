use rusqlite::{params, Connection};
use std::{error::Error, marker::PhantomData};

use crate::database::{
    errors,
    models::{Mod, NotSaved, Saved},
};

impl Mod {
    pub fn get(id: i64, db: &Connection) -> Result<Mod<Saved>, Box<dyn Error>> {
        let get_mod = include_str!("../../../../sql/mods/get_from_id.sql");

        let mut stmt = db.prepare(get_mod)?;

        let modpack = stmt.query_row(params![id], |row| {
            Ok(Mod {
                id: Some(row.get(0)?),
                project_id: row.get(1)?,
                name: row.get(2)?,
                slug: row.get(3)?,
                page_url: row.get(4)?,
                state: PhantomData::<Saved>,
            })
        })?;

        Ok(modpack)
    }
}

impl Mod<NotSaved> {
    pub fn save(self, db: &mut Connection) -> Result<Mod<Saved>, Box<dyn Error>> {
        let tx = db.transaction()?;

        let id = match tx.execute(
            include_str!("../../../../sql/mods/create.sql"),
            params![self.project_id, self.name, self.slug, self.page_url],
        ) {
            Ok(_) => tx.last_insert_rowid(),
            Err(err) => {
                if !errors::is_constraint_err(&err) {
                    return Err(err.into());
                }

                tx.query_row(
                    include_str!("../../../../sql/mods/get_from_project_id.sql"),
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

impl Mod<Saved> {
    pub fn delete(self, db: &mut Connection) -> Result<(), Box<dyn Error>> {
        let tx = db.transaction()?;

        tx.execute(
            include_str!("../../../../sql/mods/delete.sql"),
            params![self.id.unwrap()],
        )?;

        tx.commit()?;

        Ok(())
    }
}
