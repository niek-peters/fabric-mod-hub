mod mod_join;
mod modpack_join;

use derive_new::new;
use serde::Serialize;

#[derive(new, Serialize, Debug, Clone)]
pub struct ModJoin {
    pub id: i64,
    pub mod_id: i64,
    pub version_id: String,
    pub name: String,
    pub slug: String,
    pub game_version: String,
    pub page_url: String,
    pub download_url: String,
}

#[derive(new, Serialize, Clone)]
pub struct ModpackJoin {
    pub id: i64,
    pub modpack_id: i64,
    pub name: String,
    pub slug: String,
    pub game_version: String,
    pub premade: bool,
    pub installed: bool,
    pub loaded: bool,
}
