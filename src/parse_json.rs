use crate::env_var;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseJsonError {
    #[error("Error: Invalid system arch!")]
    ArchError,
    #[error("Error: Improperly formated env_add_path argument!")]
    EnvPathFormatError,
    #[error("Error: Improperly formated environment variable!")]
    EnvVarFormatError,
    #[error("Error: Improperly formated bin attr!")]
    BinFormatError,
    #[error("Error: Couldn't find version attributes!")]
    MissingVersionError,
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

pub fn get_env_variables(json_body: &serde_json::Value) -> Result<Vec<env_var::EnvVar>, ParseJsonError> {
    let mut out_vec: Vec<env_var::EnvVar> = vec![];

    if !json_body["env_set"].is_null() {
        out_vec.extend(env_var::EnvVar::from_value(&json_body["env_set"])?);
    }

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        _ => return Err(ParseJsonError::ArchError),
    };

    if !json_body["architecture"][arch]["env_set"].is_null() {
        out_vec.extend(env_var::EnvVar::from_value(
            &json_body["architecture"][arch]["env_set"],
        )?);
    }

    return Ok(out_vec);
}

fn check_bin(
    bin_attr: &serde_json::Value,
    query: &str,
) -> Result<bool, ParseJsonError> {
    if bin_attr.is_string() {
        Ok(bin_attr
            .as_str()
            .unwrap()
            .contains(query))
    } else if bin_attr.is_array() {
        let are_val_ok = bin_attr
            .as_array()
            .unwrap()
            .iter()
            .any(|val| {
                let valid_arr = if val.is_array() {
                    val.as_array()
                        .unwrap()
                        .iter()
                        .all(|val| val.is_string())
                } else {
                    false
                };
                (val.is_array() && valid_arr) || val.is_string()
            });

        if !are_val_ok {
            return Err(ParseJsonError::BinFormatError);
        }

        Ok(bin_attr
            .as_array()
            .unwrap()
            .iter()
            .any(|val| {
                if val.is_array() {
                    val.as_array()
                        .unwrap()
                        .iter()
                        .any(|alias_or_name| alias_or_name.as_str().unwrap().contains(query))
                } else {
                    val.as_str().unwrap().contains(query)
                }
            }))
    } else {
        Err(ParseJsonError::BinFormatError)
    }
}

/// checks all bins in a manifest for a certain query
pub fn query_bin(
    json_body: &serde_json::Value,
    query: &str,
) -> Result<bool, ParseJsonError> {
    if !json_body["bin"].is_null() {
        let ok_bin = match check_bin(&json_body["bin"], query) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        if ok_bin {
            return Ok(true);
        }
    }

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        _ => return Err(ParseJsonError::ArchError),
    };
    if !json_body["architecture"][arch]["bin"].is_null() {
        let ok_bin = match check_bin(&json_body["architecture"][arch]["bin"], query) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        if ok_bin {
            return Ok(true);
        }
    }

    return Ok(false);
}

pub fn get_version(json_body: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    match json_body["version"].as_str() {
        Some(out) => Ok(out.to_string()),
        None => Err(ParseJsonError::MissingVersionError.into()),
    }
}
