/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::{
    arg,
    ArgMatches,
    Command,
};
use colored::Colorize;

use crate::cli;
use crate::common::Project;

pub fn command() -> Command {
    Command::new("init")
        .about("Initializes the given project in the specified directory")
        .arg(arg!(<IDENTIFIER> "Project identifier"))
        .arg(arg!(<DIR> "The directory into which the project is to be initialized"))
        .arg_required_else_help(true)
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let identifier = matches.get_one::<String>("IDENTIFIER").unwrap();
    let dir = Path::new(matches.get_one::<String>("DIR").unwrap());

    if Project::exists_in_dir(dir) {
        anyhow::bail!("Devpost project already exists at '{}'", dir.display());
    }

    println!("Initializing new project '{}'", identifier);
    println!();

    let name = cli::prompt_required("Name: ");
    let summary = cli::prompt("Summary: ");
    let built_with = cli::prompt("Built With: ");
    let description = if cli::prompt_bool_required("Use README.md? [Y/n]: ", true) {
        Some("README.md".to_owned())
    } else {
        None
    };
    let thumbnail = "thumbnail.gif";

    let project = Project {
        name: Some(name),
        summary: Some(summary),
        built_with: built_with
            .split(',')
            .filter(|t| !t.is_empty())
            .map(|t| t.trim().into())
            .collect(),
        thumbnail: Some(thumbnail.to_owned()),
        description,
        ..Project::new(identifier.clone())
    };

    project.write_to_dir(dir)?;

    File::create(dir.join(Path::new(thumbnail)))?
        .write_all(include_bytes!("../../templates/thumbnail.gif"))?;

    if let Some(description) = project.description {
        let path = dir.join(Path::new(&description));
        if !Path::exists(&path) {
            File::create(path)?.write_all(include_bytes!("../../templates/README.md"))?;
        }
    }

    println!();
    println!(
        "{}",
        "Devpost project has been initialized successfully! 🎉"
            .bright_green()
            .bold()
    );

    Ok(())
}
