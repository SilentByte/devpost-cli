/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use clap::{
    arg,
    Command,
};

fn cli() -> Command {
    Command::new("devpost")
        .name("Devpost CLI")
        .bin_name("devpost")
        .about("An idea for a command line interface to interact with Devpost")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("login").about("Prompts you to log into your Devpost account"))
        .subcommand(Command::new("list").about("Lists all your projects"))
        .subcommand(
            Command::new("init")
                .about("Initializes the given project in the specified directory")
                .arg(arg!(<NAME> "Project name"))
                .arg(arg!(<DIR> "The directory into which the project is to be initialized"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("push").about("Uploads the changes to Devpost"))
        .subcommand(
            Command::new("preview").about("Opens the project's preview page in your browser"),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("login", sm)) => {
            unimplemented!();
        }
        Some(("init", sm)) => {
            println!(
                "INIT {} {}",
                sm.get_one::<String>("NAME").unwrap(),
                sm.get_one::<String>("DIR").unwrap(),
            );
            unimplemented!();
        }
        _ => unreachable!(),
    }
}
