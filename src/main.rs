#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

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

        match (cmd, args) {
            ("exit", "0") => process::exit(args.parse().unwrap_or(0)),
            ("echo", args) => echo(args),
            _ => println!("{}: command not found", cmd),
        }
    }
}
