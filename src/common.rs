/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use std::fs::File;
use std::path::Path;

use serde::{
    Deserialize,
    Serialize,
};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub identifier: String,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,

    #[serde(default)]
    pub built_with: Vec<String>,

    #[serde(default)]
    pub links: Vec<String>,

    #[serde(default)]
    pub images: Vec<String>,
    pub video_url: Option<String>,
}

const CONFIG_FILE: &str = "Devpost.yml";

impl Project {
    pub fn exists_in_dir(dir: &Path) -> bool {
        let dir = Path::new(dir);
        dir.join(CONFIG_FILE).exists()
    }

    pub fn from_work_dir() -> anyhow::Result<Self> {
        let file = File::open(CONFIG_FILE)?;
        let project = serde_yaml::from_reader(file)?;

        Ok(project)
    }

    pub fn new(identifier: String) -> Self {
        Project {
            identifier,
            name: None,
            summary: None,
            thumbnail: None,
            description: None,
            built_with: vec![],
            links: vec![],
            images: vec![],
            video_url: None,
        }
    }

    pub fn url(&self) -> String {
        format!("https://devpost.com/software/{}", self.identifier)
    }

    pub fn write_to_dir(&self, dir: &Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(dir)?;

        let file = File::create(dir.join(CONFIG_FILE))?;
        serde_yaml::to_writer(file, self)?;

        Ok(())
    }
}
