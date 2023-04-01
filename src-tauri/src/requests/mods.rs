use super::{ProjectResponse, ProjectVersionResponse, VersionResponse};
use crate::database::models::{Mod, Saved};
use reqwest::Client;
use std::error::Error;

impl Mod {
    pub async fn get_by_project_id(
        client: &Client,
        project_id: String,
    ) -> Result<Self, Box<dyn Error>> {
        let res = client
            .get(format!(
                "https://api.modrinth.com/v2/project/{}",
                project_id
            ))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!(
                "Mod::from_project_id: Modrinth API returned non-success status: {}",
                res.status()
            )
            .into());
        }

        let mod_res = res.json::<ProjectResponse>().await?;

        let page_url = format!("https://modrinth.com/mod/{}", project_id);

        if mod_res.project_type == "mod" {
            Ok(Mod::new(project_id, mod_res.title, mod_res.slug, page_url))
        } else {
            Err("Mod_id does not point to listed mod".into())
        }
    }

    pub async fn get_by_version_id(
        client: &Client,
        version_id: String,
    ) -> Result<Self, Box<dyn Error>> {
        let res = client
            .get(format!(
                "https://api.modrinth.com/v2/version/{}",
                version_id
            ))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!(
                "Mod::from_version_id: Modrinth API returned non-success status: {}",
                res.status()
            )
            .into());
        }

        let version_res = res.json::<VersionResponse>().await?;

        Self::get_by_project_id(client, version_res.project_id).await
    }
}

impl Mod<Saved> {
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

        let versions = res.json::<Vec<ProjectVersionResponse>>().await?;

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
