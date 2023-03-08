use derive_new::new;
use rusqlite::{params, Connection};
use std::{error::Error, marker::PhantomData};

use super::{NotSaved, Saved};

#[derive(new, Clone)]
pub struct ModpackMod<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub modpack_id: i64,
    pub mod_id: i64,
    state: PhantomData<State>,
}

impl ModpackMod {
    pub fn save(self, db: &Connection) -> Result<ModpackMod<Saved>, Box<dyn Error>> {
        let create_modpack_mod = include_str!("../../../sql/modpack_mods/create.sql");

        db.execute(create_modpack_mod, params![self.modpack_id, self.mod_id])?;

        Ok(ModpackMod {
            id: Some(self.modpack_id),
            modpack_id: self.modpack_id,
            mod_id: self.mod_id,
            state: PhantomData::<Saved>,
        })
    }
}
