use crate::database::models::Settings;
use rusqlite::Connection;
use std::{error::Error, path::PathBuf};

impl Settings {
    pub fn get_minecraft_dir(db: &Connection) -> Result<PathBuf, Box<dyn Error>> {
        let get_mc_dir = include_str!("../../../../sql/settings/get_minecraft_dir.sql");

        let mut stmt = db.prepare(get_mc_dir)?;

        let mc_dir: String = stmt.query_row([], |row| Ok(row.get(0)?))?;

        let mc_dir = PathBuf::from(mc_dir);

        Ok(mc_dir)
    }

    // pub fn get_allow_unstable(db: &Connection) -> Result<bool, Box<dyn Error>> {
    //     let get_allow_unstable = include_str!("../../../../sql/settings/get_allow_unstable.sql");

    //     let mut stmt = db.prepare(get_allow_unstable)?;

    //     let allow_unstable: bool = stmt.query_row([], |row| Ok(row.get(0)?))?;

    //     Ok(allow_unstable)
    // }

    pub fn get_allow_snapshots(db: &Connection) -> Result<bool, Box<dyn Error>> {
        let get_allow_snapshots = include_str!("../../../../sql/settings/get_allow_snapshots.sql");

        let mut stmt = db.prepare(get_allow_snapshots)?;

        let allow_snapshots: bool = stmt.query_row([], |row| Ok(row.get(0)?))?;

        Ok(allow_snapshots)
    }

    pub fn set_allow_unstable(db: &Connection, allow_unstable: bool) -> Result<(), Box<dyn Error>> {
        let set_allow_unstable = include_str!("../../../../sql/settings/set_allow_unstable.sql");

        let mut stmt = db.prepare(set_allow_unstable)?;

        stmt.execute([allow_unstable])?;

        Ok(())
    }

    pub fn set_allow_snapshots(
        db: &Connection,
        allow_snapshots: bool,
    ) -> Result<(), Box<dyn Error>> {
        let set_allow_snapshots = include_str!("../../../../sql/settings/set_allow_snapshots.sql");

        let mut stmt = db.prepare(set_allow_snapshots)?;

        stmt.execute([allow_snapshots])?;

        Ok(())
    }

    pub fn set_minecraft_dir(db: &Connection, mc_dir: PathBuf) -> Result<(), Box<dyn Error>> {
        let set_mc_dir = include_str!("../../../../sql/settings/set_minecraft_dir.sql");

        let mut stmt = db.prepare(set_mc_dir)?;

        stmt.execute([mc_dir.to_str().unwrap()])?;

        Ok(())
    }
}
