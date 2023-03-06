pub struct Modpack {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub premade: bool,
    pub removed: bool,
}

pub struct ModpackVersion {
    pub id: i32,
    pub modpack_id: i32,
    pub game_version: String,
    pub installed: bool,
}

pub struct ModpackMod {
    pub id: i32,
    pub modpack_id: i32,
    pub mod_id: i32,
}

pub struct Mod {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub slug: String,
    pub page_url: String,
}

pub struct ModVersion {
    pub id: i32,
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
