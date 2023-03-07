use derive_new::new;
use rusqlite::{params, Connection};
use std::error::Error;

use super::{modpack_mods::ModpackMod, DbModel};

#[derive(new)]
pub struct Modpack {
    #[new(default)]
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub premade: bool,
    pub mod_ids: Vec<i64>,
}

impl DbModel for Modpack {
    fn save(&self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_modpack = include_str!("../../../sql/modpacks/create.sql");

        {
            let tx = db.unchecked_transaction()?;

            tx.execute(create_modpack, params![self.name, self.slug, self.premade])?;
            let id = tx.last_insert_rowid();

            for mod_id in &self.mod_ids {
                ModpackMod::new(id, *mod_id).save(&tx)?;
            }

            tx.commit()?;
        }

        Ok(())
    }
}
