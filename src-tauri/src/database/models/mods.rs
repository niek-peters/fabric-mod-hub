use rusqlite::{params, Connection};
use std::error::Error;

use super::DbModel;

pub struct Mod {
    pub id: Option<i64>,
    pub project_id: String,
    pub name: String,
    pub slug: String,
    pub page_url: String,
}

impl DbModel for Mod {
    fn save(&self, db: &Connection) -> Result<(), Box<dyn Error>> {
        let create_mod = include_str!("../../../sql/mods/create.sql");

        db.execute(
            create_mod,
            params![self.project_id, self.name, self.slug, self.page_url],
        )?;

        Ok(())
    }
}

impl Mod {
    pub fn new(project_id: String, name: String, slug: String, page_url: String) -> Self {
        Mod {
            id: None,
            project_id,
            name,
            slug,
            page_url,
        }
    }
}
