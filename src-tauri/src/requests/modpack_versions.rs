use std::error::Error;

use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::Client;

use crate::database::models::{DbModel, Modpack, ModpackVersion};

impl Modpack {
    pub async fn get_version(
        &self,
        client: &Client,
        db: PooledConnection<SqliteConnectionManager>,
        game_version: &str,
    ) -> Result<ModpackVersion, Box<dyn Error>> {
        let id = match self.id {
            Some(id) => id,
            None => return Err("Modpack does not have an id".into()),
        };

        // Get all ModVersions from list of mod_ids
        for mod1 in &self.mods {
            let mut mod_version = mod1.get_version(client, game_version).await?;
            mod_version.save(&db)?;
        }

        Ok(ModpackVersion::new(id, game_version.to_string()))
    }
}
