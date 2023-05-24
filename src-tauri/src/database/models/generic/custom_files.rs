use rusqlite::{params, Connection};
use std::{error::Error, marker::PhantomData};

use crate::database::{
    errors,
    models::{CustomFile, NotSaved, Saved},
};

impl CustomFile<NotSaved> {
    pub fn save(self, db: &mut Connection) -> Result<CustomFile<Saved>, Box<dyn Error>> {
        let tx = db.transaction()?;

        let id = match tx.execute(
            include_str!("../../../../sql/custom_files/create.sql"),
            params![self.modpack_version_id, self.filename],
        ) {
            Ok(_) => tx.last_insert_rowid(),
            Err(err) => {
                if !errors::is_constraint_err(&err) {
                    return Err(err.into());
                }

                tx.query_row(
                    include_str!("../../../../sql/custom_files/get_from_filename.sql"),
                    params![self.filename],
                    |row| row.get(0),
                )?
            }
        };

        tx.commit()?;

        Ok(CustomFile {
            id: Some(id),
            modpack_version_id: self.modpack_version_id,
            filename: self.filename,
            state: PhantomData::<Saved>,
        })
    }
}

impl CustomFile<Saved> {
    pub fn delete(self, db: &mut Connection) -> Result<(), Box<dyn Error>> {
        let tx = db.transaction()?;

        tx.execute(
            include_str!("../../../../sql/mods/delete.sql"),
            params![self.id.unwrap()],
        )?;

        tx.commit()?;

        Ok(())
    }
}
