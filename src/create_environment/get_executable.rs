use anyhow::{Context, Ok, Result, anyhow};
use std::{path::PathBuf, process::Command};
fn find_python(version: &str) -> Result<PathBuf> {
    //python version
    let python = format!("python{}", version);
    // output
    let output = Command::new(&python)
    .arg("--version")
    .output()
    .context("failed to execute python")?;
    
    if !output.status.success() {
        return Err(anyhow!(
            "Python {version} was not found.\nPlease install Python '{version}'."
        ));
    } else {
        println!("Python {version} was found.");
        Ok(PathBuf::from(&python))
    }
}

pub fn get_python(name: &str) -> Result<PathBuf> {
    let python_path = find_python(name)?;
    println!("Python founded at '{:#?}'.", python_path);
    Ok(python_path)
}