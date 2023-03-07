use derive_new::new;
use rusqlite::{params, Connection};
use std::error::Error;

use super::DbModel;

#[derive(new)]
pub struct Mod {
    #[new(default)]
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
