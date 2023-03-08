use rusqlite::Connection;
use std::error::Error;

mod mod_versions;
mod modpack_mods;
mod modpack_versions;
mod modpacks;
mod mods;
mod settings;

pub use mod_versions::ModVersion;
// pub use modpack_mods::ModpackMod;
pub use modpack_versions::ModpackVersion;
pub use modpacks::Modpack;
pub use mods::Mod;
pub use settings::Settings;

// Organized view of the database structure
// https://drawsql.app/teams/egel-developers/diagrams/main-2

pub trait DbModel {
    fn save(&mut self, db: &Connection) -> Result<(), Box<dyn Error>>;
    // fn set_id(&mut self, id: i64);
}
