use anyhow::{Context, Result};
use std::path::Path;
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_file;

pub fn create_python_link(runtime: &Path, destination: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        // Eliminar el enlace existente si ya existe
        if destination.exists() || destination.is_symlink() {
            fs::remove_file(destination)
                .with_context(|| format!("Failed to remove existing symlink at '{}'", destination.display()))?;
        }

        symlink(runtime, destination)
            .with_context(|| {
                format!(
                    "Failed to create symlink from '{}' to '{}'",
                    runtime.display(),
                    destination.display()
                )
            })?;
    }

    #[cfg(windows)]
    {
        if destination.exists() {
            fs::remove_file(destination)
                .with_context(|| format!("Failed to remove existing file at '{}'", destination.display()))?;
        }

        symlink_file(runtime, destination)
            .with_context(|| {
                format!(
                    "Failed to create symlink from '{}' to '{}'",
                    runtime.display(),
                    destination.display()
                )
            })?;
    }

    Ok(())
}
