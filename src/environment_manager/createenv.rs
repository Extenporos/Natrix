use anyhow::{anyhow, Ok, Result};
use crate::{environment_manager::{create_dirs, create_files, get_executable}, utils::paths};
use std::{fs, io::{self, Write}};
use serde::{Deserialize, Serialize};

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
    get_executable::get_python(&version)?;
    // creating dirs
    create_dirs::create_env_folder(name)?;
    create_dirs::make_binaries_folder(name)?;
    // create files
    create_files::create_pyproject(name)?;
    create_files::create_natconf(name)?;
    // write files
    let files_path = paths::environments_dir().unwrap().join(name);
    let pyproject_path = files_path.join("pyproject.toml");
    println!("{:#?}", files_path);
     //structs
    #[derive(Debug, Serialize, Deserialize)]
    struct PyProject {
        project: Project,
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Project {
        name: String,
        version: String,
        description: String,

        #[serde(rename = "requires-python")]
        requires_python: String,
    }
    let proj = PyProject {
        project: Project {name: name.to_string(),
        version: "1.0".to_string(),
        description: "".to_string(),
        requires_python: format!(">={}", version).to_string(),
    }};
    let toml_str = toml::to_string_pretty(&proj)?;
    fs::write(&pyproject_path, toml_str)?;
    Ok(())
}