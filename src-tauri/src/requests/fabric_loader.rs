use std::error::Error;

use reqwest::Client;

use super::{LoaderJSON, LoaderVersion};

pub async fn get_json(
    game_version: &String,
    client: &Client,
) -> Result<LoaderJSON, Box<dyn Error>> {
    // Get the latest stable loader version
    let loader_version = get_loader_version(client).await?;

    // Get the JSON
    let res = client
        .get(format!(
            "https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json",
            game_version, loader_version
        ))
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(format!(
            "FabricLoader.get_json: FabricMC API returned non-success status: {}",
            res.status()
        )
        .into());
    }

    let mut json = res.json::<LoaderJSON>().await?;
    json.id = format!("FabricModHub-{}", game_version);

    Ok(json)
}

async fn get_loader_version(client: &Client) -> Result<String, Box<dyn Error>> {
    let res = client
        .get(format!("https://meta.fabricmc.net/v2/versions/loader",))
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(format!(
            "FabricLoader.get_loader_version: FabricMC API returned non-success status: {}",
            res.status()
        )
        .into());
    }

    let version = res
        .json::<Vec<LoaderVersion>>()
        .await?
        .iter()
        .find(|v| v.stable)
        .unwrap()
        .version
        .clone();

    Ok(version)
}
