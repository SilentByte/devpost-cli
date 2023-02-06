/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use clap::{
    ArgMatches,
    Command,
};

pub fn command() -> Command {
    Command::new("login").about("Prompts you to log into your Devpost account")
}

pub fn handle(matches: &ArgMatches) {
    unimplemented!()
}
