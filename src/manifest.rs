use crate::{parse_json_manifest, path::Path};

/// Struct version of a manifest.json
#[derive(Debug)]
pub struct Manifest {
    pub paths: Vec<Path>,
}

impl Manifest {
    pub fn new(manifest_value: &serde_json::Value) -> Result<Manifest, Box<dyn std::error::Error>> {
        let mut out_manifest = Manifest { paths: vec![] };

        out_manifest.paths = parse_json_manifest::find_all_bin(manifest_value)?;

        Ok(out_manifest)
    }

    pub fn from_str(manifest: &str) -> Result<Manifest, Box<dyn std::error::Error>> {
        let parsed_json: serde_json::Value = serde_json::from_str(manifest)?;
        Ok(Manifest::new(&parsed_json)?)
    }
}
