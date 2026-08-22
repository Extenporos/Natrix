use crate::commands;

pub fn handle(input: &str) {
    let mut parts = input.split_whitespace();
    let Some(command) = parts.next() else {
        return;
    };
    let arguments: Vec<&str> = parts.collect();

    if let Some(command_fn) = commands::get_command(command) {
        if let Err(error) = command_fn(&arguments) {
            eprintln!("Command failed: {error:#}"); // compatibility with anyhow
        }
    } else {
        println!("Unknown command: {}", command); // unknown command print
    }
}