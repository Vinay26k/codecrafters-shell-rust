use std::{collections::HashMap, env, fs, process};

pub enum Command {
    Exit(u8),
    Echo(String),
    Pwd,
    Cd(String),
    Type(String, HashMap<String, String>),
    OsExistingCmd(String, String, HashMap<String, String>),
}

impl Command {
    pub fn parse(command: &str, args: String, registry: HashMap<String, String>) -> Self {
        // create a command on parsing the input
        match command {
            "exit" => Command::Exit(args.parse::<u8>().unwrap()),
            "echo" => Command::Echo(args),
            "pwd" => Command::Pwd,
            "cd" => Command::Cd(args),
            "type" => Command::Type(args.to_string(), registry),
            _ => Command::OsExistingCmd(command.to_string(), args.to_string(), registry),
        }
    }

    pub fn execute(&self) {
        // executes instructions based on command
        match self {
            Command::Exit(0) => process::exit(0),
            Command::Exit(_code) => (),
            Command::Echo(message) => println!("{}", message),
            Command::Pwd => {
                if let Ok(curr_dir) = env::current_dir() {
                    println!("{}", curr_dir.to_str().unwrap())
                }
            }
            Command::Cd(path) => {
                let normalized_path = if path.starts_with("~") {
                    path.replacen("~", env::var("HOME").unwrap_or_default().as_str(), 1)
                } else if path.starts_with("./") {
                    let replacement =
                        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/";
                    path.replacen("./", &replacement, 1)
                } else if path.starts_with("../") {
                    let pwd = env::current_dir().unwrap().to_str().unwrap().to_string();
                    let mut pwd_parts = pwd.split("/").collect::<Vec<&str>>();
                    let path_parts = path.split("../").collect::<Vec<&str>>();

                    // even if character doesn't exist, splits empty string
                    let count = path_parts.len() - 1;

                    for _ in 0..count {
                        pwd_parts.pop();
                    }

                    pwd_parts.join("/")
                } else {
                    path.to_string()
                };
                // handle if we have come to root directory after multiple cd ../
                let normalized_path = if normalized_path == "" {
                    env::var("HOME").unwrap_or_default()
                } else {
                    normalized_path
                };

                let ch_cmd = env::set_current_dir(normalized_path.clone());
                match ch_cmd {
                    Ok(_) => (),
                    Err(_) => {
                        println!("cd: {}: No such file or directory", path)
                    }
                }
            }
            Command::Type(cmd, registry) => match cmd.as_str() {
                "echo" | "exit" | "pwd" | "type" => println!("{} is a shell builtin", cmd),
                _ => {
                    if registry.contains_key(cmd) {
                        let (program, program_path) = registry.get_key_value(cmd).unwrap();
                        println!("{} is {}", program, program_path)
                    } else {
                        println!("{}: not found", cmd)
                    }
                }
            },
            Command::OsExistingCmd(cmd, args, registry) => {
                // this leverages registry of commands
                if registry.contains_key(cmd) {
                    let (_program, program_path) = registry.get_key_value(cmd).unwrap();
                    let command_exec = process::Command::new(program_path)
                        .arg(args)
                        .stdout(process::Stdio::piped())
                        .stderr(process::Stdio::piped())
                        .output()
                        .unwrap();
                    print!("{}", String::from_utf8_lossy(&command_exec.stdout));
                } else {
                    println!("{}: command not found", cmd);
                }
            }
        }
    }
}

pub struct CommandCache {
    pub registry: HashMap<String, String>,
}

impl CommandCache {
    pub fn new() -> Self {
        let registry: HashMap<String, String> = HashMap::new();
        Self { registry }
    }

    fn add(&mut self, cmd: String, cmd_path: String) {
        self.registry.insert(cmd, cmd_path);
    }

    pub fn add_from_env_var(&mut self, env_var: &str) {
        for each_path in env::var(env_var).unwrap_or_default().split(":") {
            for command in fs::read_dir(each_path).unwrap() {
                if let Ok(cmd) = command {
                    self.add(
                        cmd.file_name().into_string().unwrap(),
                        fs::canonicalize(cmd.path())
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    );
                }
            }
        }
    }
}
