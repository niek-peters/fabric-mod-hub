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

impl Mod {
    pub async fn from_project_id(client: &Client, mod_id: String) -> Result<Self, Box<dyn Error>> {
        let res = client
            .get(format!("https://api.modrinth.com/v2/project/{}", mod_id))
            .send()
            .await?
            .json::<ModResponse>()
            .await?;

        let page_url = format!("https://modrinth.com/mod/{}", mod_id);

        if res.project_type == "mod" && res.status == "approved" {
            Ok(Mod::new(mod_id, res.title, res.slug, page_url))
        } else {
            Err("Mod_id does not point to listed mod".into())
        }
    }
}
