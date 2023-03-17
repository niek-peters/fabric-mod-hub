use std::error::Error;

use async_recursion::async_recursion;
use reqwest::Client;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::database::models::{Mod, ModVersion, Saved};

#[derive(Serialize, Deserialize, Debug)]
struct VersionResponse {
    id: String,
    game_versions: Vec<String>,
    version_type: String,
    status: String,
    loaders: Vec<String>,
    files: Vec<File>,
    dependencies: Vec<Dependency>,
}

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

impl Mod<Saved> {
    #[async_recursion]
    pub async fn get_version(
        &self,
        client: &Client,
        db: &mut Connection,
        game_version: &str,
        dependency_of: Option<i64>,
    ) -> Result<ModVersion<Saved>, Box<dyn Error>> {
        let id = self.id.expect("Saved mod should have an id");

        let res = client
            .get(format!(
                "https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]&loaders=[\"fabric\"]",
                self.project_id,
                game_version
            ))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!(
                "Mod.get_version: Modrinth API returned non-success status: {}",
                res.status()
            )
            .into());
        }

        let version = &res.json::<Vec<VersionResponse>>().await?[0];

        let mod_version = ModVersion::new(
            id,
            version.id.clone(),
            game_version.to_string(),
            version.files[0].url.to_string(),
            dependency_of,
        )
        .save(db)?;

        for dependency in version
            .dependencies
            .iter()
            .filter(|d| d.dependency_type != "optional")
            .cloned()
        {
            match dependency.project_id {
                Some(dependency_id) => {
                    let mod1 = Mod::from_project_id(&client, dependency_id)
                        .await?
                        .save(db)?;

                    mod1.get_version(client, db, game_version, mod_version.id)
                        .await?;
                }
                None => match dependency.version_id {
                    Some(version_id) => {
                        let mod1 = Mod::from_version_id(&client, version_id).await?.save(db)?;

                        mod1.get_version(client, db, game_version, mod_version.id)
                            .await?;
                    }
                    None => {
                        return Err("Mod.get_version: This dependency is not available".into());
                    }
                },
            }
        }

        Ok(mod_version)
    }

    pub async fn get_game_versions(&self, client: &Client) -> Result<Vec<String>, Box<dyn Error>> {
        let res = client
            .get(format!(
                "https://api.modrinth.com/v2/project/{}/version?loaders=[\"fabric\"]",
                self.project_id
            ))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!(
                "Mod.get_version: Modrinth API returned non-success status: {}",
                res.status()
            )
            .into());
        }

        let versions = res.json::<Vec<VersionResponse>>().await?;

        let mut game_versions: Vec<String> = Vec::new();
        for version in versions {
            for game_version in version.game_versions {
                if !game_versions.contains(&game_version) {
                    game_versions.push(game_version);
                }
            }
        }

        Ok(game_versions)
    }
}
