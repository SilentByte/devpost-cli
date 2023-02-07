/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use clap::{
    ArgMatches,
    Command,
};

use crate::cli;

pub fn command() -> Command {
    Command::new("login").about("Prompts you to log into your Devpost account")
}

pub fn handle(_matches: &ArgMatches) -> anyhow::Result<()> {
    cli::prompt("E-Mail: ");
    cli::prompt_password("Password: ");

    println!();
    println!("You are now logged in! 🎉");

    Ok(())
}
