/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use std::fs::File;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Project {
    identifier: String,
    name: String,
    summary: String,
    thumbnail: String,
    description: String,

    #[serde(default)]
    built_with: Vec<String>,

    #[serde(default)]
    links: Vec<String>,

    #[serde(default)]
    images: Vec<String>,
    video_url: Option<String>,
}

impl Project {
    pub fn from_dir() -> anyhow::Result<Self> {
        let file = File::open("Devpost.yml")?;
        let project = serde_yaml::from_reader(file)?;

        Ok(project)
    }

    pub fn url(&self) -> String {
        format!("https://devpost.com/software/{}", self.identifier)
    }
}
