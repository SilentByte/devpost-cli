/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use clap::{
    arg,
    ArgMatches,
    Command,
};

pub fn command() -> Command {
    Command::new("init")
        .about("Initializes the given project in the specified directory")
        .arg(arg!(<NAME> "Project name"))
        .arg(arg!(<DIR> "The directory into which the project is to be initialized"))
        .arg_required_else_help(true)
}
pub fn handle(matches: &ArgMatches) {
    println!(
        "INIT {} {}",
        matches.get_one::<String>("NAME").unwrap(),
        matches.get_one::<String>("DIR").unwrap(),
    );
    unimplemented!();
}
