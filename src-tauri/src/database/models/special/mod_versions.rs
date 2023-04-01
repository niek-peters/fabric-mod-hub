use std::{error::Error, marker::PhantomData};

use rusqlite::{params, Connection};

use crate::database::models::{ModVersion, ModpackVersion, Saved};

impl ModVersion {
    pub fn get_from_modpack_version(
        modpack_version: &ModpackVersion<Saved>,
        db: &Connection,
    ) -> Result<Vec<ModVersion<Saved>>, Box<dyn Error>> {
        let get_mod_versions =
            include_str!("../../../../sql/mod_versions/from_modpack_version_id.sql");

        let mut stmt = db.prepare(get_mod_versions)?;
        let rows = stmt.query_map(params![modpack_version.id], |row| {
            Ok(ModVersion {
                id: Some(row.get(0)?),
                mod_id: row.get(1)?,
                version_id: row.get(2)?,
                game_version: row.get(3)?,
                download_url: row.get(4)?,
                dependency_of: row.get(5)?,
                state: PhantomData::<Saved>,
            })
        })?;

        let mut mod_versions = Vec::new();

        for row in rows {
            mod_versions.push(row?);
        }

        Ok(mod_versions)
    }
}

impl ModVersion<Saved> {
    pub fn get_dependencies(
        &self,
        db: &mut Connection,
    ) -> Result<Vec<ModVersion<Saved>>, Box<dyn Error>> {
        let mut stmt = db.prepare(include_str!(
            "../../../../sql/mod_versions/get_dependencies.sql"
        ))?;

        let rows = stmt.query_map(params![self.id], |row| {
            Ok(ModVersion {
                id: Some(row.get(0)?),
                mod_id: row.get(1)?,
                version_id: row.get(2)?,
                game_version: row.get(3)?,
                download_url: row.get(4)?,
                dependency_of: row.get(5)?,
                state: PhantomData::<Saved>,
            })
        })?;

        let mut dependencies = Vec::new();

        for row in rows {
            dependencies.push(row?);
        }

        Ok(dependencies)
    }
}
