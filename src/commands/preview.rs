/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use clap::{
    ArgMatches,
    Command,
};

pub fn command() -> Command {
    Command::new("preview").about("Opens the project's preview page in your browser")
}

pub fn handle(matches: &ArgMatches) {
    unimplemented!()
}
