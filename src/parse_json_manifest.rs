use crate::path::Path;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseJsonError {
    #[error("Error: Invalid system arch!")]
    InvalidArch,
    #[error("Error: No bin path was found in manifest json!")]
    NoBinPath,
}

/// gets all binary.exes or scripts that the manifest json requested to be added to the PATH
pub fn find_all_bin(json_body: &serde_json::Value) -> Result<Vec<Path>, ParseJsonError> {
    // Locations bin could be found

    if !json_body["bin"].is_null() {
        return Ok(Path::bin_to_paths(&json_body["bin"]));
    }

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        _ => return Err(ParseJsonError::InvalidArch),
    };
    if !json_body["architecture"].is_null() {
        if !json_body["architecture"][arch].is_null() {
            return Ok(Path::bin_to_paths(&json_body["architecture"][arch]["bin"]));
        }
    }

    return Err(ParseJsonError::NoBinPath);
}
