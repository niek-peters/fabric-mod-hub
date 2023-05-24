use crate::database::{
    errors,
    models::{ModpackVersion, NotSaved, Saved},
};
use rusqlite::{params, Connection};
use std::{error::Error, marker::PhantomData};

impl ModpackVersion {
    pub fn get(db: &Connection, id: i64) -> Result<ModpackVersion<Saved>, Box<dyn Error>> {
        let get_modpack_version = include_str!("../../../../sql/modpack_versions/from_id.sql");

        let mut stmt = db.prepare(get_modpack_version)?;
        let modpack_version = stmt.query_row(params![id], |row| {
            Ok(ModpackVersion {
                id: Some(row.get(0)?),
                modpack_id: row.get(1)?,
                custom_name: row.get(2)?,
                game_version: row.get(3)?,
                installed: row.get(4)?,
                loaded: row.get(5)?,
                state: PhantomData::<Saved>,
            })
        })?;

        Ok(modpack_version)
    }
}

impl ModpackVersion<NotSaved> {
    pub fn save(self, db: &mut Connection) -> Result<ModpackVersion<Saved>, Box<dyn Error>> {
        let create_modpack_version = include_str!("../../../../sql/modpack_versions/create.sql");

        let tx = db.transaction()?;

        let id = match tx.execute(
            create_modpack_version,
            params![self.modpack_id, self.game_version],
        ) {
            Ok(_) => tx.last_insert_rowid(),
            Err(err) => {
                if !errors::is_constraint_err(&err) {
                    return Err(err.into());
                }

                tx.query_row(
                    include_str!("../../../../sql/modpack_versions/id_from_unique.sql"),
                    params![self.modpack_id, self.game_version],
                    |row| row.get(0),
                )?
            }
        };

        tx.commit()?;

        Ok(ModpackVersion {
            id: Some(id),
            modpack_id: self.modpack_id,
            custom_name: self.custom_name,
            game_version: self.game_version,
            installed: self.installed,
            loaded: self.loaded,
            state: PhantomData::<Saved>,
        })
    }
}

impl ModpackVersion<Saved> {
    pub fn update(
        self,
        custom_name: Option<String>,
        db: &mut Connection,
    ) -> Result<ModpackVersion<Saved>, Box<dyn Error>> {
        let update_modpack_version = include_str!("../../../../sql/modpack_versions/update.sql");

        let tx = db.transaction()?;

        tx.execute(
            update_modpack_version,
            params![custom_name, self.id.unwrap()],
        )?;

        tx.commit()?;

        Ok(ModpackVersion {
            id: self.id,
            modpack_id: self.modpack_id,
            custom_name,
            game_version: self.game_version,
            installed: self.installed,
            loaded: self.loaded,
            state: PhantomData::<Saved>,
        })
    }

    pub fn delete(
        self,
        app_handle: tauri::AppHandle,
        db: &mut Connection,
    ) -> Result<(), Box<dyn Error>> {
        let delete_modpack_version = include_str!("../../../../sql/modpack_versions/delete.sql");

        let tx = db.transaction()?;

        if self.loaded {
            ModpackVersion::unload_all(&tx)?;
        }

        tx.execute(delete_modpack_version, params![self.id])?;

        // Remove the modpack version directory
        self.uninstall(&app_handle)?;

        tx.commit()?;

        Ok(())
    }
}
