use std::{path::{Path, PathBuf}, process::Command};
use anyhow::{Ok};
fn validate_python_runtime(
    python: &Path,
    expected_version: &str,
) -> anyhow::Result<()> {
    let output = Command::new(python)
        .arg("--version")
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Python runtime '{}' failed to execute",
            python.display()
        );
    }

    let version = String::from_utf8_lossy(&output.stdout);

    if !version.contains(expected_version) {
        anyhow::bail!(
            "Python version mismatch: expected {}, got {}",
            expected_version,
            version.trim()
        );
    }

    Ok(())
}
pub fn find_runtime(version: &str) -> anyhow::Result<PathBuf> {
    let executable = if cfg!(windows) {
        "python.exe"
    } else {
        &format!("python{}", version)
    };
    let command = if cfg!(windows) {
        "where"
    } else {
        "which"
    };
    let output = Command::new(command)
    .arg(executable)
    .output()?;
    if !output.status.success() {
        anyhow::bail!("Python{} Runtime was not found", version);
    }
    let stdout = String::from_utf8(output.stdout)?;
    let path = stdout
    .lines()
    .next()
    .ok_or_else(|| anyhow::anyhow!("Python Path was empty"))?;
    let path = PathBuf::from(path.trim());
    if !path.exists() {
        anyhow::bail!("Python executable does not exist.");
    }
    validate_python_runtime(&path, version)?;
    Ok(path)

}
