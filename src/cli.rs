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

use indicatif::ProgressBar;

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);

    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line
}

pub fn prompt_password(prompt: impl ToString) -> String {
    rpassword::prompt_password(prompt).unwrap()
}

pub fn progress_bar(duration: Duration) {
    let pb = ProgressBar::new(100);

    for _ in 0..100 {
        pb.inc(1);
        thread::sleep(duration / 100);
    }

    pb.finish_and_clear();
}
