use derive_new::new;
use serde::Serialize;
use std::marker::PhantomData;

mod generic;
mod special;

#[derive(Clone, Debug)]
pub struct Saved;

#[derive(Clone, Debug)]
pub struct NotSaved;

#[derive(new, Serialize)]
pub struct Mod<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub project_id: String,
    pub name: String,
    pub slug: String,
    pub page_url: String,
    state: PhantomData<State>,
}

#[derive(new, Serialize)]
pub struct Modpack<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub premade: bool,
    pub mods: Vec<Mod<Saved>>,
    state: PhantomData<State>,
}

#[derive(new, Clone)]
pub struct ModpackMod<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub modpack_id: i64,
    pub mod_id: i64,
    state: PhantomData<State>,
}

#[derive(new, Serialize, Debug)]
pub struct ModVersion<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub mod_id: i64,
    pub version_id: String,
    pub game_version: String,
    pub download_url: String,
    pub dependency_of: Option<i64>,
    pub state: PhantomData<State>,
}

#[derive(new, Serialize)]
pub struct ModpackVersion<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub modpack_id: i64,
    pub game_version: String,
    #[new(default)]
    pub installed: bool,
    #[new(default)]
    pub loaded: bool,
    state: PhantomData<State>,
}

#[derive(new, Serialize)]
pub struct Settings<State = NotSaved> {
    #[new(value = "0")]
    pub id: i64,
    pub minecraft_dir: String,
    #[new(value = "false")]
    pub allow_unstable: bool,
    #[new(value = "false")]
    pub allow_snapshots: bool,
    state: PhantomData<State>,
}
