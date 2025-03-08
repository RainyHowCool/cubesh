// builtin/commands.rs
// Serve for built-in commands functions

use std::path::Path;
use std::env;
use std::process;
use std::io;
use colored::Colorize;
use crate::parser;

use crossterm::{
    execute,
    style::{Color, Print, SetForegroundColor, ResetColor},
};


pub fn builtin_echo(args: Vec<String>) {
    for arg in args.iter() {
        print!("{} ",arg);
    }
    
    println!("");
}

pub fn builtin_cd(args: Vec<String>) {
    if !args.is_empty() {
        let _ = env::set_current_dir(Path::new(&args[0]));
        unsafe {
            env::set_var("PWD", &args[0]);
        }
    } else {
        let _ = match env::var("HOME") {
            Ok(home_path) => {
                let _ = env::set_current_dir(Path::new(&home_path));
            },
            Err(_) => {
                let _ = env::set_current_dir(Path::new("/"));
                unsafe {
                    env::set_var("PWD", "/")
                }
            }
        };
    }
}

pub fn builtin_export(args: Vec<String>) {
    if !args.is_empty() {
        let new_vec: Vec<&str> = args[0].split('=').collect();

        unsafe {
            env::set_var(new_vec[0], new_vec[1]);
        }
    } else {
        for (key, value) in env::vars() {
            println!("{}={}", key, value);
        }
    }

}

pub fn builtin_exit(args: Vec<String>) {
    if args.is_empty() {
        process::exit(0);
    } else {
        match args[0].parse::<i32>() {
            Ok(errorlevel) => process::exit(errorlevel),
            Err(_) => process::exit(0)
        }
    }
}

pub fn builtin_if(mut args: Vec<String>) {
    if args.len() < 4 {
        println!("if: {}: Too few arguments. ", "error".red().bold());

        println!("\n{}", "if (built-in command) v0.1.0".bold());
        println!();
        println!("{}: ", "Usage".cyan().bold());
        println!("\tif expr1 <condition> expr2 (command)");

        println!("{}:", "Condition operators".cyan().bold());
        println!("\t==\tequals");
        println!("\t!=\tnot equals");

        println!("{}:", "Example".cyan().bold());
        println!("\texport HELLO=test\n\tif $HELLO == test echo You can see me\n");
        println!("\texport HELLO=test\n\tif $HELLO == hi echo You CANNOT see me\n");
        println!("\texport HELLO=test\n\tif $HELLO != hi echo You can see me\n");
        println!("\texport HELLO=test\n\tif $HELLO != test echo You CANNOT see me\n");

    } else {
        if args[1] == "==" { // equal
            if args[0] == args[2] {
                args.remove(2);
                args.remove(1);
                args.remove(0);

                parser::shell_exec(&args.join(" "));
            }
        } else if args[1] == "!=" {
            if args[0] != args[2] {
                args.remove(2);
                args.remove(1);
                args.remove(0);

                parser::shell_exec(&args.join(" "));
            }
        } else {
            let _ = execute!(io::stdout(),
                SetForegroundColor(Color::Red),
                Print("E: "),
                ResetColor);

            println!("Not a vaild codition: {}", args[1]);
        }
    }
}
