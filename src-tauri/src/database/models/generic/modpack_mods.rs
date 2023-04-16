use std::{error::Error, marker::PhantomData};

use rusqlite::{params, Connection};

use crate::database::{
    errors,
    models::{ModpackMod, NotSaved, Saved},
};

impl ModpackMod<NotSaved> {
    pub fn save(self, db: &Connection) -> Result<ModpackMod<Saved>, Box<dyn Error>> {
        let create_modpack_mod = include_str!("../../../../sql/modpack_mods/create.sql");

        let id = match db.execute(create_modpack_mod, params![self.modpack_id, self.mod_id]) {
            Ok(_) => db.last_insert_rowid(),
            Err(err) => {
                if !errors::is_constraint_err(&err) {
                    return Err(err.into());
                }

                db.query_row(
                    include_str!("../../../../sql/modpack_mods/id_from_unique.sql"),
                    params![self.modpack_id, self.mod_id],
                    |row| row.get(0),
                )?
            }
        };

        Ok(ModpackMod {
            id: Some(id),
            modpack_id: self.modpack_id,
            mod_id: self.mod_id,
            state: PhantomData::<Saved>,
        })
    }
}

impl ModpackMod<Saved> {
    pub fn delete(self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let delete_modpack_mod = include_str!("../../../../sql/modpack_mods/delete.sql");

        db.execute(delete_modpack_mod, params![self.id])?;

        Ok(())
    }
}
