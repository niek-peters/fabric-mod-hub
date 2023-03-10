use std::error::Error;

use derive_new::new;
use rusqlite::Connection;
use serde::Serialize;

use super::ModJoin;

#[derive(new, Serialize)]
pub struct ModpackJoin {
    pub id: i64,
    pub modpack_id: i64,
    pub name: String,
    pub slug: String,
    pub game_version: String,
    pub premade: bool,
    pub installed: bool,
    pub loaded: bool,
}

impl ModpackJoin {
    pub fn get_all(db: &Connection) -> Vec<ModpackJoin> {
        let get_all_modpacks = include_str!("../../../sql/modpack_versions/join_all.sql");

        let mut stmt = db.prepare(get_all_modpacks).unwrap();
        let modpack_joins_iter = stmt
            .query_map([], |row| {
                Ok(ModpackJoin {
                    id: row.get(0)?,
                    modpack_id: row.get(1)?,
                    name: row.get(2)?,
                    slug: row.get(3)?,
                    game_version: row.get(4)?,
                    premade: row.get(5)?,
                    installed: row.get(6)?,
                    loaded: row.get(7)?,
                })
            })
            .unwrap();

        modpack_joins_iter
            .map(|modpack_join| modpack_join.unwrap())
            .collect()
    }

    pub fn get_one(
        db: &Connection,
        modpack_version_id: i64,
    ) -> Result<ModpackJoin, Box<dyn Error>> {
        let get_one_modpack = include_str!("../../../sql/modpack_versions/join_one.sql");

        let mut stmt = db.prepare(get_one_modpack).unwrap();
        let modpack_join = stmt.query_row([modpack_version_id], |row| {
            Ok(ModpackJoin {
                id: row.get(0)?,
                modpack_id: row.get(1)?,
                name: row.get(2)?,
                slug: row.get(3)?,
                game_version: row.get(4)?,
                premade: row.get(5)?,
                installed: row.get(6)?,
                loaded: row.get(7)?,
            })
        })?;

        Ok(modpack_join)
    }
}
