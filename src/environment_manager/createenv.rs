use anyhow::{anyhow, Ok, Result};
use crate::{environment_manager::{create_dirs, create_files}, utils::paths};
use std::{fs, io::{self, Write}, process::Command};
use serde::{Deserialize, Serialize};
use configparser::ini::Ini;
use crate::runtime_manager;

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
    println!("Python Requested: '{version}'.");

    let files_path = paths::environments_dir().unwrap().join(name);
    let env_path = files_path.join(".natEnv");
    let lib_env_path = env_path.join("lib");
    let bin_env_path = env_path.join("bin");

    // creating dirs
    create_dirs::create_env_folder(name)?; // .natrix/env/test
    create_dirs::make_binaries_folder(name)?; //.natrix/env/test/.natEnv
    fs::create_dir_all(&lib_env_path)?;
    fs::create_dir_all(&bin_env_path)?;
    println!("Successfully created environment directories...");
    println!("Searching Python{} runtime...", version);
    let runtime_path = runtime_manager::get_runtime::find_runtime(version)?;
    println!("Successfully find runtime on {:#?}.", runtime_path);
    
    // searching things
     #[cfg(unix)]
    let stuff = Command::new(format!("python{}", version))
    .arg("-c")
    .arg("import sys, os; print(os.path.dirname(sys.executable)); print(sys.executable); print(sys.version.split()[0])")
    .output()?;

    #[cfg(windows)]
    let stuff = Command::new("python")
    .arg("-c")
    .arg("import sys, os; print(os.path.dirname(sys.executable)); print(sys.executable); print(sys.version.split()[0])")
    .output()?;
    
    let stdout = String::from_utf8(stuff.stdout)?;
    
    let mut lines = stdout.lines();
    let home = lines.next().unwrap_or("");
    let exec = lines.next().unwrap_or("");
    let full_version = lines.next().unwrap_or("");

    // create files
    create_files::create_pyproject(name)?;
    create_files::create_natconf(name)?;
    // pyvenv.cfg stuff
    let pyvenv_path = files_path.join("pyvenv.cfg");
    let mut file = fs::File::create(pyvenv_path)?;
    
    println!("Successfully created environment files...");
    
    // write files
    let pyproject_path = files_path.join("pyproject.toml");
     // PyProject structs
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
    // hardcoding... it works, i think.
    writeln!(file, "home = {}", home)?;
    writeln!(file, "include-system-site-packages = false")?;
    writeln!(file, "version = {}", full_version)?;
    writeln!(file, "executable = {}", exec)?;
    
     // ECF struct
    let mut ecf = Ini::new();
    let config_path = files_path.join("natConf.cfg");
    ecf.load(&config_path).unwrap();

    ecf.set("project", "version", Some("1.0.0".to_string()));
    ecf.set("project", "name", Some(name.to_string()));
    ecf.set("python", "version", Some(version.to_string()));
    ecf.set("python", "executable", Some(runtime_path.display().to_string())); // Still in development
    // making the symlink to the executable
    runtime_manager::make_runtime::create_python_link(&runtime_path, &bin_env_path.join("python3"))?;
    ecf.set("python", "runtime_path", Some(bin_env_path.join("bin").join(format!("python{}", version)).display().to_string()));
    ecf.write(config_path).unwrap();
    println!("Successfully writed files.");
    println!("Successfully created environment '{}' on {:#?}.", name, files_path);

    Ok(())
}
