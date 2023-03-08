use derive_new::new;
use rusqlite::{params, Connection};
use std::error::Error;

use super::{modpack_mods::ModpackMod, DbModel, Mod};

#[derive(new)]
pub struct Modpack {
    #[new(default)]
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub premade: bool,
    pub mods: Vec<Mod>,
}

impl DbModel for Modpack {
    fn save(&mut self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_modpack = include_str!("../../../sql/modpacks/create.sql");

        let tx = db.unchecked_transaction()?;

        tx.execute(create_modpack, params![self.name, self.slug, self.premade])?;

        let id = tx.last_insert_rowid();

        // Prepare all modpack_mods
        let mut modpack_mods: Vec<ModpackMod> = Vec::new();
        for mod1 in &self.mods {
            match mod1.id {
                Some(mod_id) => modpack_mods.push(ModpackMod::new(id, mod_id)),
                None => return Err("Mod is missing the database id".into()),
            };
        }

        // Save them to the database
        for modpack_mod in modpack_mods {
            modpack_mod.clone().save(db)?;
        }

        tx.commit()?;

        self.id = Some(id);

        Ok(())
    }
}
