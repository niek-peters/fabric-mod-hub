use crate::database::models::{Modpack, ModpackVersion, Saved};

impl Modpack<Saved> {
    pub fn get_version(&self, game_version: &str) -> ModpackVersion {
        let id = self.id.expect("Saved modpack should have an id");

        ModpackVersion::new(id, game_version.to_string())
    }
}
