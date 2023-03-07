use rusqlite::{params, Connection};
use std::error::Error;

use super::DbModel;

pub struct ModpackMod {
    pub id: Option<i64>,
    pub modpack_id: i64,
    pub mod_id: i64,
}

impl DbModel for ModpackMod {
    fn save(&self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_modpack_mod = include_str!("../../../sql/modpack_mods/create.sql");

        db.execute(create_modpack_mod, params![self.modpack_id, self.mod_id])?;

        Ok(())
    }
}

impl ModpackMod {
    pub fn new(modpack_id: i64, mod_id: i64) -> Self {
        ModpackMod {
            id: None,
            modpack_id,
            mod_id,
        }
    }
}
