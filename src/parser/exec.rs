// exec.rs
#[cfg(unix)]

use std::os::unix::process::ExitStatusExt;
use std::process::{ExitStatus, Command};

pub fn exec(name: String, arg: Vec<String>) {
    let _ = Command::new(&name)
        .args(arg)
        .status()
        .unwrap_or_else(|_| {
            println!("cubesh: {}: Command not found.", &name);
            ExitStatus::from_raw(0)
        });
}
