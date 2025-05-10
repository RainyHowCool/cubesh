// exec.rs
// This file will run the outer programs.
use colored::Colorize;
use std::process::Command;

pub fn exec(name: String, arg: Vec<String>) {
    let results = Command::new(&name).args(arg).status().unwrap_or_else();

    let _ = match results {
        Err(_) => println!("cubesh: {}: Command \"{}\" not found.",
            "error".red().bold(),
            &name),
        Ok() => todo!();
}
