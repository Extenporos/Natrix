use anyhow::{anyhow, Ok, Result};
use crate::create_environment::{get_python, create_dirs};
use std::io::{self, Write};

pub fn create_env(name: &str) -> Result<()> {
    // getting the python version
    print!("Python version? (e.g. 3.13): ");
    io::stdout().flush()?;
    let mut version = String::new();
    
    io::stdin().read_line(&mut version)?;
    let version = version.trim();
    if version.is_empty() { // checking if the user gives nothing
        return Err(anyhow!(
            "No Python version was given."
        ));
    }
    get_python::get_python(&version)?;
    // creating dirs
    create_dirs::create_env_folder(name)?;
    create_dirs::make_binaries_folder(name)?;
    Ok(())
}