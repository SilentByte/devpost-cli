/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use clap::{
    ArgMatches,
    Command,
};

pub fn command() -> Command {
    Command::new("list").about("Lists all your projects")
}

pub fn handle(matches: &ArgMatches) {
    unimplemented!()
}
