use crate::database::models::Settings;
use rusqlite::Connection;
use std::{error::Error, path::PathBuf};

impl Settings {
    pub fn get_mc_dir(db: &Connection) -> Result<PathBuf, Box<dyn Error>> {
        let get_mc_dir = include_str!("../../../../sql/settings/get_mc_dir.sql");

        let mut stmt = db.prepare(get_mc_dir)?;

        let mc_dir: String = stmt.query_row([], |row| Ok(row.get(0)?))?;

        let mc_dir = PathBuf::from(mc_dir);

        Ok(mc_dir)
    }
}
