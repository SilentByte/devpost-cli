/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

mod init;
mod list;
mod login;
mod preview;
mod push;

use clap::{
    ArgMatches,
    Command,
};

pub fn all() -> Vec<Command> {
    vec![
        login::command(),
        list::command(),
        init::command(),
        push::command(),
        preview::command(),
    ]
}

pub fn handle(name: &str, matches: &ArgMatches) {
    match name {
        "login" => login::handle(matches),
        "list" => list::handle(matches),
        "init" => init::handle(matches),
        "push" => push::handle(matches),
        "preview" => preview::handle(matches),
        _ => unimplemented!(),
    }
}
