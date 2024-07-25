use super::EnvVar;
use crate::parsing::error::ParseError;

/// Takes a serde_json::Value of type env_add_path/env_set (check scoop manifest wiki page) and converts it to Strings
/// (Mostly a helper to `get_env_paths` and `get_env_variables`)
fn env_path_to_vec(value: &serde_json::Value) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let env_path = value.as_array().ok_or_else(|| {
        Box::new(ParseError::EnvPathFormatError(format!(
            "Expected env_add_path to be a list of paths. Instead got {}",
            value
        )))
    })?;

    Ok(env_path
        .iter()
        .map(|val| match val.as_str() {
            Some(out) => Ok(out.to_string()),
            None => {
                return Err(Box::new(ParseError::EnvPathFormatError(format!(
                    "Expected env_add_path value to be string, instead found: {val}"
                ))));
            }
        })
        .collect::<Result<_, _>>()?)
}

/// Returns all env_add_path attributes from a manifest in the form of a Vec<EnvVar>
pub fn get_env_paths(
    json_body: &serde_json::Value,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut out_vec: Vec<String> = vec![];

    if !json_body["env_add_path"].is_null() {
        out_vec.extend(env_path_to_vec(&json_body["env_add_path"])?);
    };

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        arch => {
            log::error!("Current device uses an unrecognized architecutre!\nPossible architecutres are:\n\tx86\n\tx86_64\n\taarch64\nInstead got: {}\n\nExiting program",arch);
            std::process::exit(1)
        }
    };
    if !json_body["architecture"][arch]["env_add_path"].is_null() {
        out_vec.extend(env_path_to_vec(
            &json_body["architecture"][arch]["env_add_path"],
        )?);
    }

    return Ok(out_vec);
}

/// Returns all env_set attributes from a manifest in the form of a Vec<EnvVar>
pub fn get_env_variables(
    json_body: &serde_json::Value,
) -> Result<Vec<EnvVar>, Box<dyn std::error::Error>> {
    let mut out_vec: Vec<EnvVar> = vec![];

    if !json_body["env_set"].is_null() {
        out_vec.extend(EnvVar::from_value(&json_body["env_set"])?);
    }

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        arch => {
            log::error!("Current device uses an unrecognized architecutre!\nPossible architecutres are:\n\tx86\n\tx86_64\n\taarch64\nInstead got: {}\n\nExiting program",arch);
            std::process::exit(1)
        }
    };

    if !json_body["architecture"][arch]["env_set"].is_null() {
        out_vec.extend(EnvVar::from_value(
            &json_body["architecture"][arch]["env_set"],
        )?);
    }

    return Ok(out_vec);
}

/// Checks if single bin matches a query (used as helper for `query_bin`)
fn check_bin(
    bin_attr: &serde_json::Value,
    query: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if bin_attr.is_string() {
        Ok(bin_attr.as_str().unwrap().contains(query))
    } else if bin_attr.is_array() {
        let are_val_ok = bin_attr.as_array().unwrap().iter().any(|val| {
            let valid_arr = if val.is_array() {
                val.as_array().unwrap().iter().all(|val| val.is_string())
            } else {
                false
            };
            (val.is_array() && valid_arr) || val.is_string()
        });

        if !are_val_ok {
            return Err(Box::new(ParseError::BinFormatError(format!(
                "Expected bin attribute to be either a string or array, instead found: {bin_attr}"
            ))));
        }

        Ok(bin_attr.as_array().unwrap().iter().any(|val| {
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
        Err(Box::new(ParseError::BinFormatError(format!(
            "Expected bin attribute to be either a string or array, instead found: {bin_attr}"
        ))))
    }
}

/// Checks all bins in a manifest for a certain query
pub fn query_bin(
    json_body: &serde_json::Value,
    query: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
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
        arch => {
            log::error!("Current device uses an unrecognized architecutre!\nPossible architecutres are:\n\tx86\n\tx86_64\n\taarch64\nInstead got: {}\n\nExiting program",arch);
            std::process::exit(1)
        }
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

/// Returns the version of a manifest
pub fn get_version(json_body: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    match json_body["version"].as_str() {
        Some(out) => Ok(out.to_string()),
        None => Err(Box::new(ParseError::MissingVersionError)),
    }
}

/// Parses a json file from a path and returns the contents as a serde_json::Value
pub fn read_json_file(filename: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let manifest_contents = std::fs::read_to_string(filename).map_err(|e: std::io::Error| {
        Box::new(ParseError::ManifestReadError(
            filename.to_string(),
            e.to_string(),
        ))
    })?;

    let manifest_json: serde_json::Value =
        serde_json::from_str(&manifest_contents).map_err(|e| {
            Box::new(ParseError::ManifestParseError(
                filename.to_string(),
                e.to_string(),
            ))
        })?;

    Ok(manifest_json)
}
