use derive_new::new;
use reqwest::Client;
use rusqlite::{params, Connection};
use std::error::Error;
use std::marker::PhantomData;

use super::{modpack_mods::ModpackMod, Mod};

use super::{NotSaved, Saved};

#[derive(new)]
pub struct Modpack<State = NotSaved> {
    #[new(default)]
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub premade: bool,
    pub mods: Vec<Mod<Saved>>,
    state: PhantomData<State>,
}

impl Modpack<NotSaved> {
    pub fn save(self, db: &mut Connection) -> Result<Modpack<Saved>, Box<dyn Error>> {
        let create_modpack = include_str!("../../../sql/modpacks/create.sql");

        let tx = db.transaction()?;

        tx.execute(create_modpack, params![self.name, self.slug, self.premade])?;

        let id = tx.last_insert_rowid();

        // Prepare all modpack_mods
        let mut modpack_mods: Vec<ModpackMod> = Vec::new();
        for mod1 in &self.mods {
            modpack_mods.push(ModpackMod::new(
                id,
                mod1.id.expect("Saved mod should have an id"),
            ))
        }

        // Save them to the database
        for modpack_mod in modpack_mods {
            modpack_mod.clone().save(&tx)?;
        }

        tx.commit()?;

        Ok(Modpack {
            id: Some(id),
            name: self.name,
            slug: self.slug,
            premade: self.premade,
            mods: self.mods,
            state: PhantomData::<Saved>,
        })
    }

    pub async fn create(
        client: &Client,
        db: &mut Connection,
        name: String,
        slug: String,
        premade: bool,
        project_ids: Vec<String>,
    ) -> Result<Modpack<Saved>, Box<dyn Error>> {
        let mut mods: Vec<Mod<Saved>> = Vec::new();
        for project_id in project_ids {
            mods.push(Mod::from_project_id(client, project_id).await?.save(db)?);
        }

        Modpack::new(name, slug, premade, mods).save(db)
    }
}
