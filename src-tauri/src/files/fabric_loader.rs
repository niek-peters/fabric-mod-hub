use std::{error::Error, fs::File, path::PathBuf};

use reqwest::Client;

use crate::requests::fabric_loader;

pub async fn download(
    path: &PathBuf,
    game_version: String,
    client: &Client,
) -> Result<(), Box<dyn Error>> {
    // Create the directory at the path
    std::fs::create_dir_all(path)?;

    // Get the JSON
    let json = fabric_loader::get_json(&game_version, client).await?;

    // Save the JSON to a file
    let mut json_path = path.clone();
    json_path.push(format!("FabricModHub-{}.json", game_version));
    std::fs::write(json_path, serde_json::to_string_pretty(&json).unwrap())?;

    // Create the empty JAR
    let mut jar_path = path.clone();
    jar_path.push(format!("FabricModHub-{}.jar", game_version));
    File::create(jar_path)?;

    Ok(())
}
