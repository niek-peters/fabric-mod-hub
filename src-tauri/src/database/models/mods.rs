use derive_new::new;
use rusqlite::{params, Connection};
use std::{error::Error, marker::PhantomData};

use super::{NotSaved, Saved};

#[derive(new)]
pub struct Mod<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub project_id: String,
    pub name: String,
    pub slug: String,
    pub page_url: String,
    state: PhantomData<State>,
}

impl Mod<NotSaved> {
    pub fn save(self, db: &Connection) -> Result<Mod<Saved>, Box<dyn Error>> {
        let create_mod = include_str!("../../../sql/mods/create.sql");

        let tx = db.unchecked_transaction()?;

        tx.execute(
            create_mod,
            params![self.project_id, self.name, self.slug, self.page_url],
        )?;

        let id = tx.last_insert_rowid();

        tx.commit()?;

        Ok(Mod {
            id: Some(id),
            project_id: self.project_id,
            name: self.name,
            slug: self.slug,
            page_url: self.page_url,
            state: PhantomData::<Saved>,
        })
    }
}
