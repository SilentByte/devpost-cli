/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use clap::{
    ArgMatches,
    Command,
};

pub fn command() -> Command {
    Command::new("push").about("Uploads the changes to Devpost")
}

pub fn handle(matches: &ArgMatches) {
    unimplemented!()
}
