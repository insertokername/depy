use crate::path::Path;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseJsonError {
    #[error("Error: Invalid system arch!")]
    InvalidArch,
    #[error("Error: No version specified in manifest!")]
    NoVersion
}

/// gets all binary.exes or scripts that the manifest json requested to be added to the PATH
pub fn find_all_bin(json_body: &serde_json::Value) -> Result<Vec<Path>, ParseJsonError> {
    if !json_body["bin"].is_null() {
        return Ok(Path::bin_to_paths(&json_body["bin"]));
    }

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        _ => return Err(ParseJsonError::InvalidArch),
    };
    if !json_body["architecture"][arch]["bin"].is_null() {
        return Ok(Path::bin_to_paths(&json_body["architecture"][arch]["bin"]));
    }

    return Ok(vec!());
}

/// gets all added paths that the manifest json SPECIFICALLY requested to be added to the PATH
pub fn find_all_added_paths(json_body: &serde_json::Value) -> Result<Vec<Path>, ParseJsonError> {
    if !json_body["env_add_path"].is_null() {
        return Ok(Path::bin_to_paths(&json_body["env_add_path"]));
    }

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        _ => return Err(ParseJsonError::InvalidArch),
    };
    if !json_body["architecture"][arch]["env_add_path"].is_null() {
        return Ok(Path::bin_to_paths(
            &json_body["architecture"][arch]["env_add_path"],
        ));
    }

    return Ok(vec!());
}

/// get the version of a json manifes
pub fn get_version(json_body: &serde_json::Value) -> Result<String, ParseJsonError> {
    if let Some(version) = json_body["version"].as_str(){
        Ok(version.to_string())
    }
    else{
        return Err(ParseJsonError::NoVersion)
    }
}
