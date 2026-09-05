use std::{process::Command, io::{self, Write}};

fn run_external (command: &str, args: Vec<&str>) -> anyhow::Result<()> {
    let status = Command::new(command)
    .args(args)
    .status()?;

    if !status.success() {
        eprintln!("Command exited with {status}");
    }

    Ok(())
}

fn mini_handler(input: &str) -> anyhow::Result<bool> {
    let mut parts = input.split_whitespace();

    let command = match parts.next() {
        Some(command ) => command,
        None => return Ok(false),
    };

    match command {
        "cd" => {
            let path = parts.next().unwrap_or("~");
            std::env::set_current_dir(path)?;
        }

        "pwd" => {
            println!("{}", std::env::current_dir()?.display());
        }
        
        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
        }
        "exit" | "deactivate" => {
            return Ok(true);
        }

        _ => {
            run_external(command, parts.collect())?;
        }
    }

    Ok(false)
}

pub fn activate(name: &str) {
    loop {
        let current_dir = std::env::current_dir()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| "~".to_string());
        print!("{}; {} $ ", name, current_dir);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input == "\n" {
            continue;
        }
        let command = input.trim();
        if mini_handler(command).unwrap_or(false) {
            break;
        }
    }
}
