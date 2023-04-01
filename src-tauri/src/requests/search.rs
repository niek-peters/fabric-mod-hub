use super::SearchResponse;
use crate::database::models::{Mod, NotSaved};
use reqwest::Client;
use std::error::Error;

pub async fn run(client: &Client, query: String) -> Result<Vec<Mod<NotSaved>>, Box<dyn Error>> {
    let res = client
            .get(format!("https://api.modrinth.com/v2/search?query={}&facets=[[\"categories:fabric\"],[\"project_type:mod\"]]", query))
            .send()
            .await?;

    if !res.status().is_success() {
        return Err(format!(
            "Mod::from_project_id: Modrinth API returned non-success status: {}",
            res.status()
        )
        .into());
    }

    let search_res = res.json::<SearchResponse>().await?;

    Ok(search_res
        .hits
        .into_iter()
        .map(|hit| {
            let page_url = format!("https://modrinth.com/mod/{}", hit.project_id);
            Mod::new(hit.project_id, hit.title, hit.slug, page_url)
        })
        .collect())
}
