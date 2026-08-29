use anyhow::{Ok, Result};
use std::fs;
use crate::utils::paths;
use serde::{Deserialize, Serialize};

pub fn create_pyproject(path: &str) -> Result<()> {
    let i_path = paths::environments_dir()?;
    let full_path = i_path.join(path).join("pyproject.toml");
    fs::File::create(&full_path)?;
    // necessary structs
    #[derive(Debug, Serialize, Deserialize)]
    struct PyProject {
        project: Project
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Project {
        name: String,
        version: String,
        description: String,

        #[serde(rename = "requires-python")]
        requires_python: String,
    }
    Ok(())
}