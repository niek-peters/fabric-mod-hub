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
}

#[derive(Serialize, Deserialize)]
struct File {
    url: String,
}

impl Mod {
    pub async fn get_version(
        &self,
        client: &Client,
        game_version: &str,
    ) -> Result<ModVersion, Box<dyn Error>> {
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
            }
        }

        match latest_version {
            Some(version) => Ok(ModVersion::new(
                version.id,
                String::from(game_version),
                version.files[0].url.to_string(),
            )),
            None => Err("This mod version is not available".into()),
        }
    }
}
