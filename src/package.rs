use druid::{Data, Lens};
use serde::Serialize;


#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PackageError {
    #[error("Error: Invalid bucket_url format!")]
    BucketUrlFormatError,
    #[error("Error: Invalid bucket_name format!")]
    BucketNameFormatError,
    #[error("Error: Invalid name format!")]
    NameFormatError,
    #[error("Error: Invalid version format!")]
    VersionFormatError,
    #[error("Error: Invalid depy.json format!")]
    PacakgeFormatError,
}

#[derive(Data, Clone, Debug, Serialize, Lens)]
pub struct Package {
    pub bucket_url: String,
    pub bucket_name: String,
    pub name: String,
    pub version: String,
}

impl PartialEq for Package {
    fn eq(
        &self,
        other: &Package,
    ) -> bool {
        self.bucket_name.to_lowercase() == other.bucket_name.to_lowercase()
            && self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl Eq for Package {}

impl PartialOrd for Package {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Package {
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
impl Package {
    pub fn single_package_from_json(
        package_json: &serde_json::Value
    ) -> Result<Package, PackageError> {
        let package_obj = package_json
            .as_object()
            .expect("Invalid Package format, expected each package to be an package_object\n");

        let bucket_url_field = match package_obj.get("bucket_url") {
            Some(out) => out,
            None => {
                log::error!("Improper format found in package:{package_json}\nExpected bucket_url attribute in package");
                return Err(PackageError::BucketUrlFormatError);
            }
        };
        let bucket_url = match bucket_url_field.as_str() {
            Some(out) => out,
            None => {
                log::error!("Improper format found in package:{package_json}\nExpected bucket_url value to be string");
                return Err(PackageError::BucketUrlFormatError);
            }
        }
        .to_string();

        let bucket_name_field = match package_obj.get("bucket_name") {
            Some(out) => out,
            None => {
                log::error!("Improper format found in package:{package_json}\nExpected bucket_name attribute in package");
                return Err(PackageError::BucketNameFormatError);
            }
        };
        let bucket_name = match bucket_name_field.as_str() {
            Some(out) => out,
            None => {
                log::error!("Improper format found in package:{package_json}\nExpected bucket_name value to be string");
                return Err(PackageError::BucketNameFormatError);
            }
        }
        .to_string();

        let name_field = match package_obj.get("name") {
            Some(out) => out,
            None => {
                log::error!("Improper format found in package:{package_json}\nExpected name attribute in package");
                return Err(PackageError::NameFormatError);
            }
        };
        let name = match name_field.as_str() {
            Some(out) => out,
            None => {
                log::error!("Improper format found in package:{package_json}\nExpected name value to be string");
                return Err(PackageError::NameFormatError);
            }
        }
        .to_string();

        let version_field = match package_obj.get("version") {
            Some(out) => out,
            None => {
                log::error!("Improper format found in package:{package_json}\nExpected version attribute in package");
                return Err(PackageError::VersionFormatError);
            }
        };
        let version = match version_field.as_str() {
            Some(out) => out,
            None => {
                log::error!("Improper format found in package:{package_json}\nExpected version value to be string");
                return Err(PackageError::VersionFormatError);
            }
        }
        .to_string();
        Ok(Package {
            bucket_url,
            bucket_name,
            name,
            version,
        })
    }

    pub fn multiple_packages_from_json(
        json: &serde_json::Value
    ) -> Result<Vec<Package>, Box<dyn std::error::Error>> {
        if let Some(out) = json.as_array() {
            let temp = out
                .into_iter()
                .map(|pkg| Package::single_package_from_json(pkg).map_err(|e| Box::new(e)))
                .collect::<Result<Vec<Package>, Box<PackageError>>>();
            Ok(temp?)
        } else {
            log::error!("Invalid install json, expected the installer to be an array of packages!");
            return Err(Box::new(PackageError::PacakgeFormatError));
        }
    }

    pub fn save_packages_to_json(
        packages: &Vec<Package>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let str_package = serde_json::to_string_pretty(packages).unwrap();
        std::fs::write("./depy.json", str_package).unwrap();
        Ok(())
    }
}
