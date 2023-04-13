use serde::{Deserialize, Serialize};

pub mod fabric_loader;
pub mod mod_versions;
pub mod modpack_versions;
pub mod mods;
pub mod search;

// //project/{project_id}
#[derive(Serialize, Deserialize)]
pub struct ProjectResponse {
    title: String,
    slug: String,
    project_type: String,
    status: String,
}

// /version/{version_id}
#[derive(Serialize, Deserialize)]
pub struct VersionResponse {
    project_id: String,
}

// /project/{project_id}/version
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectVersionResponse {
    id: String,
    game_versions: Vec<String>,
    version_type: String,
    status: String,
    loaders: Vec<String>,
    files: Vec<File>,
    dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    hits: Vec<Hit>,
}

#[derive(Serialize, Deserialize)]
pub struct LoaderVersion {
    version: String,
    stable: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoaderJSON {
    id: String,
    inherits_from: String,
    release_time: String,
    time: String,
    r#type: String,
    main_class: String,
    arguments: Arguments,
    libraries: Vec<Library>,
}

// Sub-structs of the different responses
#[derive(Serialize, Deserialize, Debug)]
struct File {
    url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Dependency {
    project_id: Option<String>,
    version_id: Option<String>,
    dependency_type: String,
}

#[derive(Serialize, Deserialize)]
struct Hit {
    project_id: String,
    title: String,
    slug: String,
}

#[derive(Serialize, Deserialize)]
struct Arguments {
    game: Vec<String>,
    jvm: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Library {
    name: String,
    url: String,
}
