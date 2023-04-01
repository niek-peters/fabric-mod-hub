use super::ProjectVersionResponse;
use crate::database::models::{Mod, ModVersion, Saved};
use async_recursion::async_recursion;
use reqwest::Client;
use rusqlite::Connection;
use std::error::Error;

impl ModVersion {
    #[async_recursion]
    pub async fn get_from_mod(
        mod1: &Mod<Saved>,
        client: &Client,
        db: &mut Connection,
        game_version: &str,
        dependency_of: Option<i64>,
    ) -> Result<ModVersion<Saved>, Box<dyn Error>> {
        let id = mod1.id.expect("Saved mod should have an id");

        let res = client
            .get(format!(
                "https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]&loaders=[\"fabric\"]",
                mod1.project_id,
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

        let version = &res.json::<Vec<ProjectVersionResponse>>().await?[0];

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
                    let mod1 = Mod::get_by_project_id(&client, dependency_id)
                        .await?
                        .save(db)?;

                    ModVersion::get_from_mod(&mod1, client, db, game_version, mod_version.id)
                        .await?;
                }
                None => match dependency.version_id {
                    Some(version_id) => {
                        let mod1 = Mod::get_by_version_id(&client, version_id)
                            .await?
                            .save(db)?;

                        ModVersion::get_from_mod(&mod1, client, db, game_version, mod_version.id)
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
}
