use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn natrix_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow!("could not determine the home directory"))?;

    Ok(home.join(".natrix"))
}

pub fn environments_dir() -> Result<PathBuf> {
    Ok(natrix_dir()?.join("env"))
}
