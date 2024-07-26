use error::ParseError;

/// Contains the ParseError enum
pub mod error;
/// Json reading and transforming it to Manifest
pub mod parse_json;

/// Struct version of a manifest.json stripped down to only the necessary attributes
#[derive(Debug, PartialEq)]
pub struct Manifest {
    pub version: String,
    pub name: String,
    pub url: String,
    pub added_paths: Vec<String>,
    pub env_vars: Vec<EnvVar>,
}

impl Manifest {
    pub fn new(
        manifest_value: &serde_json::Value,
        name: String,
        url: String,
        version: String,
    ) -> Result<Manifest, Box<dyn std::error::Error>> {
        let added_paths = parse_json::get_env_paths(manifest_value)
            .map_err(|err| ParseError::ManifestCreateError(err.to_string()))?;
        let env_vars = parse_json::get_env_variables(manifest_value)
            .map_err(|err| ParseError::ManifestCreateError(err.to_string()))?;
        Ok(Manifest {
            version,
            name,
            url,
            added_paths,
            env_vars,
        })
    }

    /// Transforms a str of a json into a serde_json::Value and then creates a Manifest
    pub fn from_str(
        manifest: &str,
        name: String,
        url: String,
        version: String,
    ) -> Result<Manifest, Box<dyn std::error::Error>> {
        let parsed_json: serde_json::Value = serde_json::from_str(manifest)
            .map_err(|err| ParseError::ManifestCreateError(err.to_string()))?;
        Ok(Manifest::new(&parsed_json, name, url, version)?)
    }
}

/// Struct version of the env_set field in a scoop manifest
#[derive(Debug, PartialEq)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

impl EnvVar {
    pub fn new(name: String, value: String) -> EnvVar {
        EnvVar { name, value }
    }

    /// Transforms a serde val into a vec of environment variables
    pub fn from_value(
        value: &serde_json::Value,
    ) -> Result<Vec<EnvVar>, Box<dyn std::error::Error>> {
        let env_var_iter = value
            .as_object()
            .ok_or_else(|| {
                ParseError::EnvVariableFormatError(format!(
                    "environemnt variables are supposed to be a dictionary/object. Instead got {}",
                    value
                ))
            })?
            .into_iter();

        let env_var_list = env_var_iter
            .map(|val| {
                let second = val.1.as_str();
                match second {
                    Some(second) => Ok((val.0.clone(), second.to_string())),
                    None => Err(ParseError::EnvVariableFormatError(format!(
                        "environemnt variables is supposed to be a string. Instead got {:?}",
                        val
                    ))),
                }
            })
            .collect::<Result<Vec<(String, String)>, _>>()?
            .into_iter()
            .map(|val| EnvVar::new(val.0, val.1))
            .collect::<Vec<EnvVar>>();

        Ok(env_var_list)
    }
}
