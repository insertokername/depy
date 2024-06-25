use crate::{env_var, parse_json};

/// Struct version of a manifest.json
#[derive(Debug, PartialEq)]
pub struct Manifest {
    pub version: String,
    pub name: String,
    pub added_paths: Vec<String>,
    pub env_vars: Vec<env_var::EnvVar>,
}

impl Manifest {
    pub fn new(
        manifest_value: &serde_json::Value,
        name: String,
        version: String,
    ) -> Result<Manifest, Box<dyn std::error::Error>> {
        let added_paths = parse_json::find_all_added_paths(manifest_value)?;
        let env_vars = parse_json::get_env_variables(manifest_value)?;
        Ok(Manifest {
            version,
            name,
            added_paths,
            env_vars,
        })
    }

    pub fn from_str(
        manifest: &str,
        name: String,
        version: String,
    ) -> Result<Manifest, Box<dyn std::error::Error>> {
        let parsed_json: serde_json::Value = serde_json::from_str(manifest)?;
        Ok(Manifest::new(&parsed_json, name, version)?)
    }
}
