use include_sqlite_sql::{impl_sql, include_sql};
use rusqlite::Connection;
include_sql!("sql/modpacks.sql");

pub struct Modpack {
    pub id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub premade: bool,
    pub removed: bool,
}

impl Modpack {
    pub fn new(db: Connection, name: String, slug: String, premade: Option<bool>, mods: Vec<Mod>) {
        let premade = premade.unwrap_or(false);
        let removed = false;

        let modpack = Modpack {
            id: None,
            name,
            slug,
            premade,
            removed,
        };

        let modpack_id =
            db.create_modpack(&modpack.name, &modpack.slug, &modpack.premade, |id| Ok(id));
    }
}

pub struct ModpackVersion {
    pub id: Option<i32>,
    pub modpack_id: i32,
    pub game_version: String,
    pub installed: bool,
}

pub struct ModpackMod {
    pub id: Option<i32>,
    pub modpack_id: i32,
    pub mod_id: i32,
}

pub struct Mod {
    pub id: Option<i32>,
    pub project_id: i32,
    pub name: String,
    pub slug: String,
    pub page_url: String,
}

pub struct ModVersion {
    pub id: Option<i32>,
    pub mod_id: i32,
    pub modpack_version_id: i32,
    pub version_id: i32,
    pub game_version: String,
    pub download_url: String,
    pub installed: bool,
}

pub struct Settings {
    pub id: i32,
    pub minecraft_dir: String,
    pub stable_only: bool,
}
