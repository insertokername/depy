use self::error::ShellError;
use std::os::windows::process::CommandExt;

/// Bucket related shell operations
pub mod bucket;
/// Cleanup / Deletion related shell operations
pub mod cleanup;
/// Directory reading/modifing related shell operations
pub mod dir;
/// Contains the ShellError enum
pub mod error;
/// Installation related shell operations
pub mod install;
/// Virtual environment related shell operations
pub mod venv;

/// runs generic command inside the depy/scoop folder
pub fn run_cmd_in_depy_dir(cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("cmd")
        .arg("/C")
        .arg(cmd)
        .env("SCOOP", dir::get_depy_scoop_location()?)
        .creation_flags(0x08000000)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Box::new(ShellError::ExecutionError(stderr.to_string())));
    }

    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}
