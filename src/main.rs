use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, process};

fn echo(st: &str) {
    println!("{}", st);
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut parts = input.trim().splitn(2, ' ');
        let cmd = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("");

        let commands = ["exit", "echo", "type"];
        let mut all_commands: HashMap<String, String> = HashMap::new();

        for each_path in env::var("PATH").unwrap().split(":") {
            for cmd in fs::read_dir(each_path).unwrap() {
                if let Ok(cmd_str) = cmd {
                    let file_name = cmd_str.file_name().into_string().unwrap();
                    let file_path = fs::canonicalize(cmd_str.path())
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    all_commands.insert(file_name, file_path);
                }
            }
        }

        for cmd in commands {
            all_commands.insert(cmd.to_string(), String::from("a shell builtin"));
        }
        match (cmd, args) {
            ("exit", "0") => process::exit(args.parse().unwrap()),
            ("echo", args) => echo(args),
            ("type", args) => {
                if all_commands.contains_key(&args.to_string()) {
                    let (program, path) = all_commands.get_key_value(args).unwrap();
                    println!("{} is {}", program, path)
                } else {
                    println!("{}: not found", &args)
                }
            }
            _ => {
                if all_commands.contains_key(&cmd.to_string()) {
                    let (_program, cmd_path) = all_commands.get_key_value(cmd).unwrap();
                    let output = process::Command::new(cmd_path)
                        .stdout(process::Stdio::piped())
                        .stderr(process::Stdio::piped())
                        .arg(args)
                        .output()
                        .unwrap();
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    println!("{}: command not found", cmd);
                }
            }
        }
    }
}
