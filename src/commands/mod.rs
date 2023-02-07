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
use colored::Colorize;

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
    let result = match name {
        "login" => login::handle(matches),
        "list" => list::handle(matches),
        "init" => init::handle(matches),
        "push" => push::handle(matches),
        "preview" => preview::handle(matches),
        _ => unimplemented!(),
    };

    match result {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(e) => {
            println!("{} {:?}", "error:".bright_red().bold(), e);
            std::process::exit(1);
        }
    }
}
