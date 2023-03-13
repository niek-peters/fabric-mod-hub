use derive_new::new;
use reqwest::Client;
use rusqlite::{params, Connection, Result};
use serde::Serialize;
use std::error::Error;
use std::marker::PhantomData;

use crate::database::errors;

use super::{modpack_mods::ModpackMod, Mod};

use super::{ModpackVersion, NotSaved, Saved};

#[derive(new, Serialize)]
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

        let id = match tx.execute(create_modpack, params![self.name, self.slug, self.premade]) {
            Ok(_) => tx.last_insert_rowid(),
            Err(err) => {
                if !errors::is_constraint_err(&err) {
                    return Err(err.into());
                }

                tx.query_row(
                    include_str!("../../../sql/modpacks/id_from_slug.sql"),
                    params![self.slug],
                    |row| row.get(0),
                )?
            }
        };

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

impl Modpack<Saved> {
    pub async fn create_version(
        self,
        client: &Client,
        db: &mut Connection,
        game_version: &str,
    ) -> Result<ModpackVersion<Saved>, Box<dyn Error>> {
        let id = self.id.expect("Saved modpack should have an id");

        // Get all ModVersions from modpack's list of mods and save them
        for mod1 in &self.mods {
            mod1.get_version(client, db, game_version, None).await?;
        }

        Ok(ModpackVersion::new(id, game_version.to_string()).save(db)?)
    }
}

impl Modpack {
    pub fn get_all(db: &Connection) -> Vec<Modpack<Saved>> {
        let get_modpacks = include_str!("../../../sql/modpacks/get.sql");

        let mut stmt = db.prepare(get_modpacks).unwrap();

        let modpack_iter = stmt
            .query_map(params![], |row| {
                Ok(Modpack {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    slug: row.get(2)?,
                    premade: row.get(3)?,
                    mods: Vec::new(),
                    state: PhantomData::<Saved>,
                })
            })
            .unwrap();

        modpack_iter.map(|modpack| modpack.unwrap()).collect()
    }
}
