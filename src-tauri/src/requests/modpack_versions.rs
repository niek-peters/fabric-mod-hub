use crate::database::models::{Modpack, Saved};
use reqwest::Client;
use std::error::Error;

impl Modpack<Saved> {
    pub async fn get_game_versions(&self, client: &Client) -> Result<Vec<String>, Box<dyn Error>> {
        // Get all the game_versions from the modpack's mods that are available for all the mods
        let mut game_versions = Vec::new();

        for mod1 in &self.mods {
            let mod_game_versions = mod1.get_game_versions(client).await?;

            if game_versions.is_empty() {
                game_versions = mod_game_versions;
            } else {
                game_versions.retain(|v| mod_game_versions.contains(v));
            }
        }

        Ok(game_versions)
    }
}
