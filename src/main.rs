use std::io::{self, Write};

// local modules
mod command;
use command::{Command, CommandCache};

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
        let args = parts.next().unwrap_or("").to_string();

        let mut command_registry = CommandCache::new();
        command_registry.add_from_env_var("PATH");

        let command = Command::parse(cmd, args, command_registry.registry);
        command.execute()
    }
}
