use serde::{Deserialize, Serialize};

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
struct ProjectVersionResponse {
    id: String,
    game_versions: Vec<String>,
    version_type: String,
    status: String,
    loaders: Vec<String>,
    files: Vec<File>,
    dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize)]
struct SearchResponse {
    hits: Vec<Hit>,
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
