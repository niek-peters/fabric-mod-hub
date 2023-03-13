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
                "https://api.modrinth.com/v2/project/{}/version",
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

        let version_res = res.json::<Vec<VersionResponse>>().await?;

        let mut latest_version: Option<VersionResponse> = None;
        for version in version_res {
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
            Some(version) => {
                let mod_version = ModVersion::new(
                    id,
                    version.id,
                    game_version.to_string(),
                    version.files[0].url.to_string(),
                    dependency_of,
                )
                .save(db)?;

                let dependencies: Vec<Dependency> = version
                    .dependencies
                    .iter()
                    .filter(|d| d.dependency_type != "optional")
                    .cloned()
                    .collect();

                for dependency in dependencies {
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
                                let mod1 =
                                    Mod::from_version_id(&client, version_id).await?.save(db)?;

                                mod1.get_version(client, db, game_version, mod_version.id)
                                    .await?;
                            }
                            None => {
                                return Err(
                                    "Mod.get_version: This dependency is not available".into()
                                );
                            }
                        },
                    }
                }

                Ok(mod_version)
            }
            None => Err("Mod.get_version: This mod version is not available".into()),
        }
    }
}
