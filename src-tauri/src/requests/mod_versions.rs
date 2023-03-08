use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::database::models::{Mod, ModVersion};

#[derive(Serialize, Deserialize)]
struct VersionResponse {
    id: String,
    game_versions: Vec<String>,
    version_type: String,
    status: String,
    loaders: Vec<String>,
    files: Vec<File>,
    dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize)]
struct File {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Dependency {
    version_id: String,
}

impl Mod {
    pub async fn get_version(
        &self,
        client: &Client,
        game_version: &str,
    ) -> Result<ModVersion, Box<dyn Error>> {
        let id = match self.id {
            Some(id) => id,
            None => return Err("Mod does not have an id".into()),
        };

        let res = client
            .get(format!(
                "https://api.modrinth.com/v2/project/{}/version",
                self.project_id
            ))
            .send()
            .await?
            .json::<Vec<VersionResponse>>()
            .await?;

        let mut latest_version: Option<VersionResponse> = None;
        for version in res {
            if version.game_versions.contains(&game_version.to_string())
                && version.version_type == "release"
                && version.status == "listed"
                && version.loaders.contains(&"fabric".to_string())
            {
                latest_version = Some(version);
                break;
            }
        }

        match latest_version {
            Some(version) => Ok(ModVersion::new(
                id,
                version.id,
                game_version.to_string(),
                version.files[0].url.to_string(),
                version
                    .dependencies
                    .iter()
                    .map(|d| d.version_id.to_string())
                    .collect(),
            )),
            None => Err("This mod version is not available".into()),
        }
    }
}
