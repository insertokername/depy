use crate::env_var::EnvVar;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseJsonError {
    #[error("Error: Invalid system arch!")]
    ArchError,
    #[error("Error: Improperly formated env_add_path argument!")]
    EnvPathFormatError,
    #[error("Error: Improperly formated environment variable!")]
    EnvVarFormatError,
}

fn env_path_to_vec(value: &serde_json::Value) -> Result<Vec<String>, ParseJsonError> {
    let env_path = match value.as_array() {
        Some(out) => out,
        None => {
            log::error!(
                "Expected env_add_path to be a list of paths. Instead got {}",
                value
            );
            return Err(ParseJsonError::EnvPathFormatError);
        }
    };

    Ok(env_path
        .iter()
        .map(|val| {
            let out = match val.as_str() {
                Some(out) => out,
                None => {
                    log::error!("Expected env_add_path value to be string, instead found: {val}");
                    return Err(ParseJsonError::EnvPathFormatError);
                }
            };
            Ok(out.to_string())
        })
        .collect::<Result<Vec<String>, ParseJsonError>>()?)
}

/// gets all added paths that the manifest json SPECIFICALLY requested to be added to the PATH
pub fn find_all_added_paths(json_body: &serde_json::Value) -> Result<Vec<String>, ParseJsonError> {
    let mut out_vec: Vec<String> = vec![];

    if !json_body["env_add_path"].is_null() {
        out_vec.extend(env_path_to_vec(&json_body["env_add_path"])?);
    };

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        _ => return Err(ParseJsonError::ArchError),
    };
    if !json_body["architecture"][arch]["env_add_path"].is_null() {
        out_vec.extend(env_path_to_vec(
            &json_body["architecture"][arch]["env_add_path"],
        )?);
    }

    return Ok(out_vec);
}

pub fn get_env_variables(json_body: &serde_json::Value) -> Result<Vec<EnvVar>, ParseJsonError> {
    let mut out_vec: Vec<EnvVar> = vec![];

    if !json_body["env_set"].is_null() {
        out_vec.extend(EnvVar::from_value(&json_body["env_set"])?);
    }

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        _ => return Err(ParseJsonError::ArchError),
    };

    if !json_body["architecture"][arch]["env_set"].is_null() {
        out_vec.extend(
            EnvVar::from_value(&json_body["architecture"][arch]["env_set"])?,
        );
    }

    return Ok(out_vec);
}
