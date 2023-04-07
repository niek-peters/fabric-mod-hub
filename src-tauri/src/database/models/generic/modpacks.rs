use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::marker::PhantomData;

use crate::database::errors;
use crate::database::models::{Mod, Modpack, ModpackMod, NotSaved, Saved};

impl Modpack {
    pub fn get(db: &Connection, id: i64) -> Result<Modpack<Saved>, Box<dyn Error>> {
        let get_modpack = include_str!("../../../../sql/modpacks/get_from_id.sql");

        let mut stmt = db.prepare(get_modpack)?;

        let mods = Mod::get_by_modpack_id(id, db)?;

        let modpack = stmt.query_row(params![id], |row| {
            Ok(Modpack {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                slug: row.get(2)?,
                premade: row.get(3)?,
                mods,
                state: PhantomData::<Saved>,
            })
        })?;

        Ok(modpack)
    }

    pub fn get_all(db: &Connection) -> Vec<Modpack<Saved>> {
        let get_modpacks = include_str!("../../../../sql/modpacks/get.sql");

        let mut stmt = db.prepare(get_modpacks).unwrap();

        let modpack_iter = stmt
            .query_map(params![], |row| {
                let id = row.get(0)?;
                let mods = Mod::get_by_modpack_id(id, db).unwrap();

                Ok(Modpack {
                    id: Some(id),
                    name: row.get(1)?,
                    slug: row.get(2)?,
                    premade: row.get(3)?,
                    mods,
                    state: PhantomData::<Saved>,
                })
            })
            .unwrap();

        modpack_iter.map(|modpack| modpack.unwrap()).collect()
    }
}

impl Modpack<NotSaved> {
    pub fn save(self, db: &mut Connection) -> Result<Modpack<Saved>, Box<dyn Error>> {
        let create_modpack = include_str!("../../../../sql/modpacks/create.sql");

        let tx = db.transaction()?;

        let id = match tx.execute(create_modpack, params![self.name, self.slug, self.premade]) {
            Ok(_) => tx.last_insert_rowid(),
            Err(err) => {
                if !errors::is_constraint_err(&err) {
                    return Err(err.into());
                }

                tx.query_row(
                    include_str!("../../../../sql/modpacks/id_from_slug.sql"),
                    params![self.slug],
                    |row| row.get(0),
                )?
            }
        };

        // Prepare all modpack_mods
        for mod1 in &self.mods {
            ModpackMod::new(id, mod1.id.expect("Saved mod should have an id")).save(&tx)?;
        }

        tx.commit()?;

        Ok(Modpack {
            id: Some(id),
            name: self.name,
            slug: self.slug,
            premade: self.premade,
            mods: self.mods,
            state: PhantomData::<Saved>,
        })
    }
}

impl Modpack<Saved> {
    pub fn delete(self, db: &mut Connection) -> Result<(), Box<dyn Error>> {
        let delete_modpack = include_str!("../../../../sql/modpacks/delete.sql");

        db.execute(delete_modpack, params![self.id.unwrap()])?;

        Ok(())
    }
}
