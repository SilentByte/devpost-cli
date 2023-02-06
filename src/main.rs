/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use clap::Command;

fn cli() -> Command {
    Command::new("devpost")
        .name("Devpost CLI")
        .bin_name("devpost")
        .about("An idea for a command line interface to interact with Devpost")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(devpost_cli::commands::all())
}

fn main() {
    match cli().get_matches().subcommand() {
        Some((name, matches)) => devpost_cli::commands::handle(name, matches),
        _ => unreachable!(),
    }
}
