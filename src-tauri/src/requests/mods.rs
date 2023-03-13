use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::database::models::Mod;

#[derive(Serialize, Deserialize)]
struct ModResponse {
    title: String,
    slug: String,
    project_type: String,
    status: String,
}

#[derive(Serialize, Deserialize)]
struct VersionResponse {
    project_id: String,
}

impl Mod {
    pub async fn from_project_id(client: &Client, mod_id: String) -> Result<Self, Box<dyn Error>> {
        let res = client
            .get(format!("https://api.modrinth.com/v2/project/{}", mod_id))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!(
                "Mod::from_project_id: Modrinth API returned non-success status: {}",
                res.status()
            )
            .into());
        }

        let mod_res = res.json::<ModResponse>().await?;

        let page_url = format!("https://modrinth.com/mod/{}", mod_res.slug);

        if mod_res.project_type == "mod" && mod_res.status == "approved" {
            Ok(Mod::new(mod_id, mod_res.title, mod_res.slug, page_url))
        } else {
            Err("Mod_id does not point to listed mod".into())
        }
    }

    pub async fn from_version_id(
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

        Self::from_project_id(client, version_res.project_id).await
    }
}
