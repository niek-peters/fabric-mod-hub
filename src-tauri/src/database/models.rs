use rusqlite::Connection;
use std::error::Error;

pub mod mod_versions;
pub mod modpack_mods;
pub mod modpack_versions;
pub mod modpacks;
pub mod mods;

pub mod settings;

pub trait DbModel {
    fn save(&self, db: &Connection) -> Result<(), Box<dyn Error>>;
}
