use crate::{env_var::EnvVar, parse_json_manifest, path::Path};

/// Struct version of a manifest.json
#[derive(Debug, PartialEq)]
pub struct Manifest {
    pub version: String,
    pub name: String,
    pub bin_paths: Vec<Path>,
    pub added_paths: Vec<Path>,
    pub env_vars: Vec<EnvVar>,
}

impl Manifest {
    pub fn new(manifest_value: &serde_json::Value, name : String) -> Result<Manifest, Box<dyn std::error::Error>> {
        let bin_paths = parse_json_manifest::find_all_bin(manifest_value)?;
        let added_paths = parse_json_manifest::find_all_added_paths(manifest_value)?;
        let env_vars = parse_json_manifest::get_env_variables(manifest_value)?;
        let version = parse_json_manifest::get_version(manifest_value)?;
        Ok(Manifest {
            version,
            name,
            bin_paths,
            added_paths,
            env_vars,
        })
    }

    pub fn from_str(manifest: &str, name: String) -> Result<Manifest, Box<dyn std::error::Error>> {
        let parsed_json: serde_json::Value = serde_json::from_str(manifest)?;
        Ok(Manifest::new(&parsed_json, name)?)
    }
}
