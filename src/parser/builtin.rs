// builtin.rs
// Serve Shell Built-in commands

mod commands;

pub fn builtin_check(cmd: &String) -> bool {
    // TODO: Must rewrite the Array!!
    let builtin_list = [
        "echo".to_string(),
        "cd".to_string(),
        "export".to_string(),
        "exit".to_string(),
        "if".to_string(),
    ];

    builtin_list.contains(&cmd)
}

pub fn builtin_exec(cmd: String, arg: Vec<String>) {
    let cmd: &str = &cmd;

    match cmd {
        "echo" => commands::builtin_echo(arg),
        "cd" => commands::builtin_cd(arg),
        "export" => commands::builtin_export(arg),
        "exit" => commands::builtin_exit(arg),
        "if" => commands::builtin_if(arg),
        _ => todo!(),
    }
}
