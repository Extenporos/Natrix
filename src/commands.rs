use anyhow::{Context, Ok, Result}; // error management 
use std::{collections::HashMap}; // registry for commands
use console::{Style, Term}; // styles and others things
use crate::create_environment::createenv; //create env module

mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

type CommandFn = fn(&[&str]) -> Result<()>;
// built-in commands lol
fn version(_arguments: &[&str]) -> Result<()> { //version command
    let version_style = Style::new().bold().cyan();
    println!("{}", version_style.apply_to("Natrix v1.1.0 'Rust Re-Write'"));
    Ok(()) //success value, if not success, then raise an error with anyhow
}

fn about(_arguments: &[&str]) -> Result<()> { // about command
    println!(r#"
    About Natrix

    Natrix
    Version {}

    A fast and lightweight package and runtime manager.

    Developed by Vortex
    Compiled with Rust {}

    Build Information
    • Target: {}
    • Profile: {}
    • Build date: {}
    • Git commit: {}

    Copyright © 2026 VortexNN
    Licensed under the MIT License

    Thanks for using Natrix.
    "#,
    build_info::PKG_VERSION,
    build_info::RUSTC_VERSION,
    build_info::TARGET,
    build_info::PROFILE,
    build_info::BUILT_TIME_UTC,
    build_info::GIT_COMMIT_HASH.unwrap_or("unknown"),
);
    Ok(())
}

fn clear(_arguments: &[&str]) -> Result<()> { //clear command
    Term::stdout().clear_screen()?;
    Ok(())
}

fn create(arguments: &[&str]) -> Result<()> { //create environment command
    let Some(nameenv) = arguments.first() else {
        println!("The environment name wasn't given");
        return Ok(());
    };
    createenv::create_env(nameenv).context("could not create environment")?;
    Ok(())
}

pub fn command_map() -> HashMap<&'static str, CommandFn> { //registry function
    let mut commands: HashMap<&'static str, CommandFn> = HashMap::new(); // makes the registry
    commands.insert("version", version); // register the command
    commands.insert("about", about);
    commands.insert("clear", clear);
    commands.insert("create", create);
    commands //idk why this exists
}

pub fn get_command(command: &str) -> Option<CommandFn> {
    command_map().get(command).copied()
}
