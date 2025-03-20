// parser.rs
use regex::Regex;
use std::env;

mod builtin;
mod exec;

struct Command {
    name: String,
    arg: Vec<String>,
}

impl Command {
    fn new(nameset: String, argset: Vec<String>) -> Self {
        // Init Data
        Self {
            name: nameset,
            arg: argset,
        }
    }
}

fn split_one_command(input: String) -> Command {
    let mut cmdlist: Vec<String> = split_special(input, ' ');

    Command::new(cmdlist[0].clone(), {
        cmdlist.remove(0);
        cmdlist
    })
}

fn split_commands(input: String) -> Vec<Command> {
    let cmds: Vec<String> = split_special(input, '&');
    let mut codes: Vec<Command> = vec![];
    for cmd in cmds.iter() {
        codes.push(split_one_command((&cmd).to_string()));
    }

    codes
}

fn exec_command(cmd: &Command) {
    if builtin::builtin_check(&cmd.name.clone()) {
        builtin::builtin_exec(cmd.name.clone(), cmd.arg.clone());
    } else {
        exec::exec(cmd.name.clone(), cmd.arg.clone());
    }
}

fn split_special(input: String, delimiter: char) -> Vec<String> {
    // Special Split
    let pattern = format!(r#"([^{}]+)|("[^"]*")"#, delimiter);
    let re = Regex::new(&pattern).unwrap();

    re.captures_iter(&input)
        .filter_map(|cap| {
            cap.get(1).or(cap.get(2)).map(|m| {
                let mut tmp = m.as_str().to_string();
                if tmp.chars().next() == Some('$') {
                    tmp.remove(0);
                    tmp = match env::var(tmp) {
                        Ok(value) => value,
                        Err(_) => "".to_string(),
                    };
                }
                tmp
            })
        })
        .collect() // Return Value
}

pub fn shell_exec(input: &str) {
    for onecmd in split_commands(input.to_string()).iter() {
        exec_command(onecmd);
    }
}
