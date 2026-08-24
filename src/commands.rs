use anyhow::{Context, Ok, Result}; // error management 
use std::collections::HashMap; // registry for commands
use console::{Style, Term}; // styles and others things
use crate::createenv::create_env; //create env module

type CommandFn = fn(&[&str]) -> Result<()>;

fn version(_arguments: &[&str]) -> Result<()> { //version command
    let version_style = Style::new().bright().cyan();
    println!("{}", version_style.apply_to("Natrix v1.1.0 'Rust Re-Write'"));
    Ok(()) //success value, if not success, then raise an error with anyhow
}

fn about(_arguments: &[&str]) -> Result<()> { // about command
    println!("Natrix Development Test...");
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
    create_env(nameenv).context("could not create environment")?;
    Ok(())
}
fn test(arguments: &[&str]) -> Result<()> {
    if arguments.is_empty() {
        println!("No argument was given")
    } else {
        println!("Arguments were given: {}", arguments.join(" "))
    }
    Ok(())
}

pub fn command_map() -> HashMap<&'static str, CommandFn> { //registry function
    let mut commands: HashMap<&'static str, CommandFn> = HashMap::new(); // makes the registry
    commands.insert("version", version); // register the command
    commands.insert("about", about);
    commands.insert("clear", clear);
    commands.insert("create", create);
    commands.insert("test", test);
    commands //idk why this exists
}

pub fn get_command(command: &str) -> Option<CommandFn> {
    command_map().get(command).copied()
}
