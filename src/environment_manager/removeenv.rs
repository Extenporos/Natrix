use crate::utils::paths;
use anyhow::Result;
use std::fs;

pub fn remove_env(name: &str) -> Result<()> {
    println!("Fetching name...");
    let env_path = paths::environments_dir().unwrap().join(name);
    println!("Removing '{}'...", name);
    fs::remove_dir_all(env_path)?;
    println!("Successfully removed '{}'.", name);

    Ok(())
}
