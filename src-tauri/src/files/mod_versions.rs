use crate::database::models::{ModVersion, Saved};

use reqwest::Client;
use std::{
    error::Error,
    fs::File,
    io::{self, Cursor},
    path::PathBuf,
};
use uuid::Uuid;

impl ModVersion<Saved> {
    pub async fn download(&self, path: &PathBuf, client: &Client) -> Result<(), Box<dyn Error>> {
        let name = Uuid::new_v4().to_string();

        let res = client.get(&self.download_url).send().await?;
        let (_, name) = res.url().path().rsplit_once('/').unwrap_or(("", &name));

        let mut path_copy = path.clone();
        path_copy.push(name);

        let bytes = res.bytes().await?;
        let mut file = File::create(path_copy)?;
        let mut content = Cursor::new(bytes);
        io::copy(&mut content, &mut file)?;

        Ok(())
    }
}
