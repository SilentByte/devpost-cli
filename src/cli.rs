/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use std::io::{
    stdin,
    stdout,
    Write,
};
use std::thread;
use std::time::Duration;

use colored::Colorize;
use indicatif::ProgressBar;

pub fn prompt(text: &str) -> String {
    print!("{}", text.bold());

    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim_matches(|c| c == '\r' || c == '\n').into()
}

pub fn prompt_required(text: &str) -> String {
    let mut input;

    loop {
        input = prompt(text);
        if !input.is_empty() {
            break;
        }
    }

    input
}

pub fn prompt_bool_required(text: &str, default: bool) -> bool {
    loop {
        let input = prompt(text);
        let input = input.trim();

        if input.is_empty() {
            return default;
        }

        if input == "y" {
            return true;
        }

        if input == "n" {
            return false;
        }
    }
}

pub fn prompt_password(prompt: &str) -> String {
    rpassword::prompt_password(prompt.bold()).unwrap()
}

pub fn progress_bar(duration: Duration) {
    let pb = ProgressBar::new(100);

    for _ in 0..100 {
        pb.inc(1);
        thread::sleep(duration / 100);
    }

    pb.finish_and_clear();
}
