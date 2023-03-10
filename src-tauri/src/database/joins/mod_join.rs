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
                    version_id: row.get(1)?,
                    name: row.get(3)?,
                    slug: row.get(4)?,
                    game_version: row.get(5)?,
                    page_url: row.get(5)?,
                    download_url: row.get(6)?,
                })
            })
            .unwrap();

        mod_joins_iter
            .map(|mod_join| modpack_join.unwrap())
            .collect()
    }
}