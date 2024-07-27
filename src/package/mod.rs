//! This  module is the struct representation of a scoop package
//!
//! Mostly used as an output of parsing the depy.json file, which outputs 
//! a `Vec<Package>` through the `multiple_packages_from_json` function.
//! And also used as an input for the `shell::install::install` function.
//! 
//! # Example 
//! ```
//! // This function just throws prettier errors, you could manually read a json file with serde_json
//! let json_value = parsing::parse_json::read_json_file("./depy.json")?;
//! 
//! let packages = package::multiple_packages_from_json(&json_value)?;
//! 
//! shell::install::install(packages)?;
//! ```

use druid::{Data, Lens};
use error::PackageError;
use serde::Serialize;

/// Contains the PackageError enum
pub mod error;

/// Struct representation of a generic scoop package
#[derive(Data, Clone, Debug, Serialize, Lens)]
pub struct Package {
    pub bucket_url: String,
    pub bucket_name: String,
    pub name: String,
    pub version: String,
}

impl PartialEq for Package {
    fn eq(&self, other: &Package) -> bool {
        self.bucket_name.to_lowercase() == other.bucket_name.to_lowercase()
            && self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl Eq for Package {}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

/// Recieves a serde_json::Value and parses it into a single package
pub fn single_package_from_json(
    package_json: &serde_json::Value,
) -> Result<Package, Box<dyn std::error::Error>> {
    let package_obj = package_json
        .as_object()
        .ok_or_else(|| Box::new(PackageError::PackageStructureError))?;

    let bucket_url_field = package_obj.get("bucket_url").ok_or_else(||{
                log::error!("Improper format found in package:{package_json}\nExpected bucket_url attribute in package");
                return Box::new(PackageError::BucketUrlFormatError);
        })?;

    let bucket_url = bucket_url_field.as_str().ok_or_else(||{
                log::error!("Improper format found in package:{package_json}\nExpected bucket_url value to be string");
                return Box::new(PackageError::BucketUrlFormatError);
        })?
        .to_string();

    let bucket_name_field = package_obj.get("bucket_name").ok_or_else(||{
                log::error!("Improper format found in package:{package_json}\nExpected bucket_name attribute in package");
                return Box::new(PackageError::BucketNameFormatError);
        })?;

    let bucket_name = bucket_name_field.as_str().ok_or_else(||{
                log::error!("Improper format found in package:{package_json}\nExpected bucket_name value to be string");
                return Box::new(PackageError::BucketNameFormatError);
        })?
        .to_string();

    let name_field = package_obj.get("name").ok_or_else(|| {
        log::error!(
            "Improper format found in package:{package_json}\nExpected name attribute in package"
        );
        return Box::new(PackageError::NameFormatError);
    })?;

    let name = name_field
        .as_str()
        .ok_or_else(|| {
            log::error!(
                "Improper format found in package:{package_json}\nExpected name value to be string"
            );
            return Box::new(PackageError::NameFormatError);
        })?
        .to_string();

    let version_field = package_obj.get("version").ok_or_else(||{
                log::error!("Improper format found in package:{package_json}\nExpected version attribute in package");
                return Box::new(PackageError::VersionFormatError);
        })?;

    let version = version_field.as_str().ok_or_else(||{
                log::error!("Improper format found in package:{package_json}\nExpected version value to be string");
                return Box::new(PackageError::VersionFormatError);
        })?
        .to_string();
    Ok(Package {
        bucket_url,
        bucket_name,
        name,
        version,
    })
}

/// Recieves a serde_json::Value and parses it into a single package
pub fn multiple_packages_from_json(
    json: &serde_json::Value,
) -> Result<Vec<Package>, Box<dyn std::error::Error>> {
    if let Some(out) = json.as_array() {
        let temp = out
            .into_iter()
            .map(|pkg| single_package_from_json(pkg).map_err(|e| e))
            .collect::<Result<Vec<Package>, Box<dyn std::error::Error>>>();
        Ok(temp?)
    } else {
        log::error!("Invalid install json, expected the installer to be an array of packages!");
        return Err(Box::new(PackageError::PacakgeFormatError));
    }
}

/// Saves a list of packages to ./depy.json
pub fn save_packages_to_json(packages: &Vec<Package>) -> Result<(), Box<dyn std::error::Error>> {
    let str_package = serde_json::to_string_pretty(packages).unwrap();
    std::fs::write("./depy.json", str_package)
        .map_err(|err| PackageError::PacakgeSaveError(err.to_string()))?;
    Ok(())
}
