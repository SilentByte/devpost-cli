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
    Command::new("login").about("Prompts you to log into your Devpost account")
}

pub fn handle(_matches: &ArgMatches) -> anyhow::Result<()> {
    println!(
        "{}",
        include_str!("../../templates/banner.txt")
            .bright_white()
            .on_blue()
    );

    println!();
    println!("Enter your credentials for Devpost.com");
    println!();

    cli::prompt_required("E-Mail:   ");
    cli::prompt_password("Password: ");

    cli::progress_bar(Duration::from_millis(200));

    println!();
    println!("{}", "You are now logged in! 🎉".bright_green().bold());

    Ok(())
}
