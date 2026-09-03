use anyhow::{Ok, Result};
use std::fs;
use crate::utils::paths;

pub fn create_pyproject(path: &str) -> Result<()> {
    let i_path = paths::environments_dir()?;
    let full_path = i_path.join(path).join("pyproject.toml");
    fs::File::create(&full_path)?;
    Ok(())
}
pub fn create_natconf(path: &str) -> Result<()> {
    let i_path = paths::environments_dir()?;
    let full_path = i_path.join(path).join("natConf.cfg");
    fs::File::create(&full_path)?;
    Ok(())
}
