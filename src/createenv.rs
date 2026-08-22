use anyhow::{Context, Result};
use std::fs; // files and folders
use std::io::Write; // for write files
use std::path::PathBuf;

pub fn create_env(name: &str) -> Result<()> {
    let mut env = PathBuf::from(".natrix"); // .natrix directory idk where it is but default is root of executable
    /* fs::create_dir_all(&env).context("could not create .natrix directory")?; commented because when you run natrix for first time, creates the .natrix directory
    and because env.push and the env create dir exists makes the env directory with .natrix lol*/

    env.push("env");
    fs::create_dir_all(&env).context("could not create environment directory")?; //.natrix/env directory
    env.push(name);
    fs::create_dir(&env).context("could not create environment")?;
    // make files code below
    env.push("natrixEnv.cfg");
    let mut conf_file = fs::File::create(&env)?;
    conf_file.write_all(b"test\n")?;

    Ok(())
}