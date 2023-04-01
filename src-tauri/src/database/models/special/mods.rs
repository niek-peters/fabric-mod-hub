use rusqlite::{params, Connection};
use std::{error::Error, marker::PhantomData};

use crate::database::models::{Mod, Saved};

impl Mod {
    pub fn get_by_modpack_id(
        modpack_id: i64,
        db: &Connection,
    ) -> Result<Vec<Mod<Saved>>, Box<dyn Error>> {
        let mut stmt = db.prepare(include_str!("../../../../sql/mods/get_from_modpack_id.sql"))?;
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
