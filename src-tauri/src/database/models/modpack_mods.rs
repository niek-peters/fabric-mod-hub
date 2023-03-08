use derive_new::new;
use rusqlite::{params, Connection};
use std::error::Error;

use super::DbModel;

#[derive(new, Clone)]
pub struct ModpackMod {
    #[new(default)]
    pub id: Option<i64>,
    pub modpack_id: i64,
    pub mod_id: i64,
}

impl DbModel for ModpackMod {
    fn save(&mut self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_modpack_mod = include_str!("../../../sql/modpack_mods/create.sql");

        db.execute(create_modpack_mod, params![self.modpack_id, self.mod_id])?;

        self.id = Some(self.modpack_id);

        Ok(())
    }
}
