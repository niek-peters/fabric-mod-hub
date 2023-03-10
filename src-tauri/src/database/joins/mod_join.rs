use std::error::Error;

use derive_new::new;
use rusqlite::Connection;
use serde::Serialize;

#[derive(new, Serialize)]
pub struct ModJoin {
    pub id: i64,
    pub mod_id: i64,
    pub version_id: String,
    pub name: String,
    pub slug: String,
    pub game_version: String,
    pub page_url: String,
    pub download_url: String,
}

impl ModJoin {
    pub fn get_all(db: &Connection) -> Vec<ModJoin> {
        let get_all_mods = include_str!("../../../sql/mod_versions/join_all.sql");

        let mut stmt = db.prepare(get_all_mods).unwrap();
        let mod_joins_iter = stmt
            .query_map([], |row| {
                Ok(ModJoin {
                    id: row.get(0)?,
                    mod_id: row.get(1)?,
                    version_id: row.get(2)?,
                    name: row.get(3)?,
                    slug: row.get(4)?,
                    game_version: row.get(5)?,
                    page_url: row.get(5)?,
                    download_url: row.get(6)?,
                })
            })
            .unwrap();

        mod_joins_iter.map(|mod_join| mod_join.unwrap()).collect()
    }

    pub fn get_from_modpack_version_id(
        db: &Connection,
        modpack_version_id: i64,
    ) -> Result<Vec<ModJoin>, Box<dyn Error>> {
        let get_all_mods =
            include_str!("../../../sql/mod_versions/join_from_modpack_version_id.sql");

        let mut stmt = db.prepare(get_all_mods).unwrap();
        let mod_joins_iter = stmt
            .query_map([modpack_version_id], |row| {
                Ok(ModJoin {
                    id: row.get(0)?,
                    mod_id: row.get(1)?,
                    version_id: row.get(2)?,
                    name: row.get(3)?,
                    slug: row.get(4)?,
                    game_version: row.get(5)?,
                    page_url: row.get(5)?,
                    download_url: row.get(6)?,
                })
            })
            .unwrap();

        let mod_joins = mod_joins_iter
            .map(|mod_join| mod_join.unwrap())
            .collect::<Vec<ModJoin>>();

        Ok(mod_joins)
    }

    pub fn get_one(db: &Connection, mod_version_id: i64) -> Result<ModJoin, Box<dyn Error>> {
        let get_one_mod = include_str!("../../../sql/mod_versions/join_one.sql");

        let mut stmt = db.prepare(get_one_mod).unwrap();
        let mod_join = stmt.query_row([mod_version_id], |row| {
            Ok(ModJoin {
                id: row.get(0)?,
                mod_id: row.get(1)?,
                version_id: row.get(2)?,
                name: row.get(3)?,
                slug: row.get(4)?,
                game_version: row.get(5)?,
                page_url: row.get(5)?,
                download_url: row.get(6)?,
            })
        })?;

        Ok(mod_join)
    }
}
