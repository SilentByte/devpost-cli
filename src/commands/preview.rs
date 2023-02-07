/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use clap::{
    ArgMatches,
    Command,
};

use crate::common::Project;

pub fn command() -> Command {
    Command::new("preview").about("Opens the project's preview page in your browser")
}

pub fn handle(_matches: &ArgMatches) -> anyhow::Result<()> {
    let project = Project::from_dir()?;

    open::that(project.url())?;

    Ok(())
}
