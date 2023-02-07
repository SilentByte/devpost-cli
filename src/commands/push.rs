/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use std::time::Duration;

use clap::{
    ArgMatches,
    Command,
};
use colored::Colorize;

use crate::cli;

pub fn command() -> Command {
    Command::new("push").about("Uploads the changes to Devpost")
}

pub fn handle(_matches: &ArgMatches) -> anyhow::Result<()> {
    println!("Uploading changes to devpost.com...");

    cli::progress_bar(Duration::from_secs(2));

    println!("{}", "Your changes are now live! 🎉".bright_green().bold());

    Ok(())
}
