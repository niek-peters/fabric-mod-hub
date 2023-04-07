use std::error::Error;

use reqwest::Client;
use rusqlite::{params, Connection};

use crate::database::models::{Mod, Modpack, Saved};

impl Modpack {
    pub async fn create(
        client: &Client,
        db: &mut Connection,
        name: String,
        slug: String,
        premade: bool,
        project_ids: Vec<String>,
    ) -> Result<Modpack<Saved>, Box<dyn Error>> {
        let mut mods: Vec<Mod<Saved>> = Vec::new();
        for project_id in project_ids {
            mods.push(Mod::get_by_project_id(client, project_id).await?.save(db)?);
        }

        Modpack::new(name, slug, premade, mods).save(db)
    }

    pub fn slug_exists(db: &Connection, slug: &str) -> Result<bool, Box<dyn Error>> {
        let slug_exists = include_str!("../../../../sql/modpacks/get_from_slug.sql");

        let mut stmt = db.prepare(slug_exists)?;
        let exists = stmt.exists(params![slug])?;

        Ok(exists)
    }
}
