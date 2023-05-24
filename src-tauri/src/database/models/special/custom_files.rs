use std::{error::Error, marker::PhantomData};

use rusqlite::{params, Connection};

use crate::database::models::{CustomFile, Saved};

impl CustomFile {
    pub fn get_from_modpack_version_id(
        modpack_version_id: i64,
        db: &Connection,
    ) -> Result<Vec<CustomFile<Saved>>, Box<dyn Error>> {
        let get_custom_files =
            include_str!("../../../../sql/custom_files/get_from_modpack_version_id.sql");

        let mut stmt = db.prepare(get_custom_files)?;
        let rows = stmt.query_map(params![modpack_version_id], |row| {
            Ok(CustomFile {
                id: Some(row.get(0)?),
                modpack_version_id: row.get(1)?,
                filename: row.get(2)?,
                state: PhantomData::<Saved>,
            })
        })?;

        let mut custom_files = Vec::new();

        for row in rows {
            custom_files.push(row?);
        }

        Ok(custom_files)
    }
}
