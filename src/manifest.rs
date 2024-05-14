use crate::path::Path;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ManifestError {
    #[error("Error: Invalid system arch!")]
    InvalidArch,
    #[error("Error: No bin path was found in manifest!")]
    NoBinPath,
}

/// Struct version of a manifest.json
#[derive(Debug)]
pub struct Manifest {
    pub paths: Vec<Path>,
    json_body: serde_json::Value,
}

impl Manifest {
    /// gets all binary.exes or scripts that the manifest requested to be added to the PATH
    fn find_all_bin(&self) -> Result<Vec<Path>, ManifestError> {
        // Locations bin could be found

        if !self.json_body["bin"].is_null() {
            return Ok(Path::bin_to_paths(&self.json_body["bin"]));
        }

        let arch = match std::env::consts::ARCH {
            "x86" => "32bit",
            "x86_64" => "64bit",
            "aarch64" => "arm64",
            _ => return Err(ManifestError::InvalidArch),
        };
        if !self.json_body["architecture"].is_null() {
            if !self.json_body["architecture"][arch].is_null() {
                return Ok(Path::bin_to_paths(
                    &self.json_body["architecture"][arch]["bin"],
                ));
            }
        }

        return Err(ManifestError::NoBinPath);
    }

    pub fn new(manifest: &serde_json::Value) -> Result<Manifest, Box<dyn std::error::Error>> {
        let mut out_manifest = Manifest {
            json_body: manifest.clone(),
            paths: vec![],
        };

        out_manifest.paths = out_manifest.find_all_bin()?;

        Ok(out_manifest)
    }

    pub fn from_str(manifest: &str) -> Result<Manifest, Box<dyn std::error::Error>> {
        let parsed_json: serde_json::Value = serde_json::from_str(manifest)?;
        Ok(Manifest::new(&parsed_json)?)
    }
}
