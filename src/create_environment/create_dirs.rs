use std::fs;
use anyhow::{Context, Ok, Result};
use crate::utils::paths;

pub fn create_env_folder(path: &str) -> Result<()> {
    let env_path = paths::environments_dir()?;
    let full_path = env_path.join(path);
    fs::create_dir_all(&full_path).context("could not create environment folder")?;
    Ok(())
}
pub fn make_binaries_folder(name: &str) -> Result<()> {
    let bin_path = paths::environments_dir()?;
    let full_path = bin_path.join(name).join(".natEnv");
    fs::create_dir_all(&full_path).context("could not create bin folder")?;
    Ok(())
}