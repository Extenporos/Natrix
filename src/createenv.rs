use anyhow::{anyhow, Context, Result};
use std::fs; // files and folders
use std::io::{self, Write}; // for write files
use configparser::ini::Ini;
use std::{process::Command, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::utils::paths;
pub fn create_env(name: &str) -> Result<()> {
    let mut env = paths::environments_dir()?;
    /* fs::create_dir_all(&env).context("could not create .natrix directory")?; commented because when you run natrix for first time, creates the .natrix directory
    and because env.push and the env create dir exists makes the env directory with .natrix lol*/
    //request python version
    print!("Python Version? (e.g. 3.13): ");
    io::stdout().flush()?;
    
    let mut version = String::new(); //string of python version
    
    io::stdin().read_line(&mut version)?; //saves version string
    
    let version = version.trim();
    let python = format!("python{}", version);

    println!("Python requested: {version}");

    let result = Command::new(&python)
        .arg("--version")
        .output();
    
    match result { // checks if the requested python version exists
        Ok(output) if output.status.success() => {
            println!("Found {python}");
        }

        _ => {
            return Err(anyhow!(
                "Python {version} was not found.\nPlease install Python '{version}'."
            ));
        }
    }
    #[cfg(windows)]
    let executable_path = Command::new("where")
        .arg(&python)
        .output()?;

    #[cfg(unix)]
    let executable_path = Command::new("which")
        .arg(&python)
        .output()?;

    let executable_path = PathBuf::from(
        String::from_utf8(executable_path.stdout)
            .context("could not read Python executable path")?
            .trim(),
    );

    // create folders
    fs::create_dir_all(&env).context("could not create env folder")?; //.natrix/env directory
    env.push(name);
    if env.exists() { // if exists will use this error
        return Err(anyhow!(
            "environment '{}' already exists. Use 'remove {}' first",
            name,
            name,
        ));
    }
    fs::create_dir(&env)?;
    env.push(".natfiles");
    fs::create_dir(&env)?;
    //copy the python executable
    // make files code below
    env.pop();
    env.push("pyproject.toml"); // pyproject toml path
    
    fs::File::create_new(&env)
        .context("could not create pyproject.toml file")?;
    
    #[derive(Debug, Deserialize, Serialize)]
    struct PyProyect { // pyproject struct 
        project: Project,
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    struct Project { // project struct
        name: String,
        version: String,
        description: String,

        #[serde(rename = "requires-python")]
        requires_python: String,
    }
    let pyprojectdata = PyProyect {
    project: Project {
        name: name.to_string(),
        version: "0.1.0".to_string(),
        description: String::new(),
        requires_python: format!(">={}", version).to_string(),
        },
    };
    
    let content = toml::to_string_pretty(&pyprojectdata)?;
    println!("Config content (pyproject):\n{}", content);
    println!("You can change the config content on {:?}.", env);
    fs::write(&env, content)
        .context("could not write pyproject.toml")?;

    //TODO:  make code to create more folders and files like natEnv.cfg and others
    let mut natconf = Ini::new();
    env.pop();
    let mut conf_path = PathBuf::from(env); conf_path.push("natEnv.cfg");

    natconf.set("natenv", "version", Some("1".to_string()));
    natconf.set("natenv", "envname", Some(name.to_string()));
    
    natconf.set("python", "version", Some(version.to_string()));
    natconf.set("python", "implementation", Some("CPython".to_string()));
    natconf.set("python", "executable", Some("test".to_string()));
    /*
    [natenv]
    version = 1
    envname = test
    [python]
    version = 3.13
    implementation = CPython
    */
    natconf.write(&conf_path)
    .context("could not write Natrix Environment Config file")?;

    Ok(())
}
