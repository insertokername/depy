use crate::env_var::EnvVar;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseJsonError {
    #[error("Error: Invalid system arch!")]
    InvalidArch,
}

/// gets all added paths that the manifest json SPECIFICALLY requested to be added to the PATH
pub fn find_all_added_paths(json_body: &serde_json::Value) -> Result<Vec<String>, ParseJsonError> {
    let mut out_vec: Vec<String> = vec![];

    if !json_body["env_add_path"].is_null() {
        out_vec.extend(
            json_body["env_add_path"]            
            .as_array()
            .expect(&format!("env_add_path in {json_body} is not formated corectly!\n"))
            .iter()
            .map(|val| val.as_str().expect(&format!("Expected env_add_path value to be string, instead found: {val}, in manifest {json_body}")).to_string())
            .collect::<Vec<String>>()
            );
    }

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        _ => return Err(ParseJsonError::InvalidArch),
    };
    if !json_body["architecture"][arch]["env_add_path"].is_null() {
        out_vec.extend(
            json_body["architecture"][arch]["env_add_path"]
                .as_array()
                .expect(&format!("env_add_path in {json_body} is not formated corectly!\n"))
                .iter()
                .map(|val| val.as_str().expect(&format!("Expected env_add_path value to be string, instead found: {val}, in manifest {json_body}")).to_string())
                .collect::<Vec<String>>()
        );
    }

    return Ok(out_vec);
}

pub fn get_env_variables(json_body: &serde_json::Value) -> Result<Vec<EnvVar>, ParseJsonError> {
    let mut out_vec: Vec<EnvVar> = vec![];

    if !json_body["env_set"].is_null() {
        out_vec.extend(EnvVar::from_multiple_values(&json_body["env_set"]).unwrap());
    }

    let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
        _ => return Err(ParseJsonError::InvalidArch),
    };

    if !json_body["architecture"][arch]["env_set"].is_null() {
        out_vec.extend(
            EnvVar::from_multiple_values(&json_body["architecture"][arch]["env_set"]).unwrap(),
        );
    }

    return Ok(out_vec);
}
