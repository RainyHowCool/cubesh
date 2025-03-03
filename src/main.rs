use std::io;
use std::io::Write;

mod parser;
mod keyboard;

fn main() {
    loop {
        // 1. Put Prompt
        print!("$ ");
        let _ = io::stdout().flush(); // Flush buffer or can't show prompt

        // 2. Get User Input
        let mut input = String::new();
        keyboard::get_line(&mut input).unwrap();
        
        let input = input.trim();

        // 3. Parse input
        parser::shell_exec(input);
    }
}
