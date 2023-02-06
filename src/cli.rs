/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use std::io::{
    stdin,
    stdout,
    Write,
};

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);

    stdout().flush();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line
}

pub fn prompt_password(prompt: impl ToString) -> String {
    rpassword::prompt_password(prompt).unwrap()
}
