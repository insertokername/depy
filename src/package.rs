use druid::{im::Vector, Data};

use crate::parse_json_manifest;

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
}

#[derive(Data, Clone, Debug)]
pub struct Package {
    pub bucket: String,
    pub bucket_name: String,
    pub name: String,
    pub version: String,
}

impl Package {
    pub fn from_value(package_json: &serde_json::Value) -> Result<Package, PackageError> {
        // println!("{:#?}",package);
        let package_obj = package_json
            .as_object()
            .expect("Invalid Package format, expected each package to be an package_object\n");

        let bucket_field = match package_obj.get("bucket_url") {
            Some(out) => out,
            None => {
                log::error!("Improper format found in package:{package_json}\nExpected bucket_url attribute in package");
                return Err(PackageError::BucketUrlFormatError);
            }
        };
        let bucket = match bucket_field.as_str() {
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
            bucket,
            bucket_name,
            name,
            version,
        })
    }

    pub fn query_local_buckets(query: &str) -> Result<Vector<Package>, Box<dyn std::error::Error>> {
        let mut out_vect = Vector::<Package>::new();

        let buckets = std::fs::read_dir(crate::dir::get_depy_dir_location() + "\\buckets")
            .expect("Couldn't find depy installation");

        for bucket in buckets {
            let ok_bucket = match bucket {
                Ok(val) => val,
                Err(err) => return Err(err.into()),
            };

            let manifests = std::fs::read_dir(ok_bucket.path().join("bucket"));

            let ok_manifests = match manifests {
                Ok(val) => val,
                Err(_) => {
                    println!("Found invalid bucket at:{:#?}", ok_bucket);
                    continue;
                }
            };

            out_vect.extend(
                ok_manifests
                    .filter_map(|val| val.ok())
                    .filter_map(|val| {
                        let filename = val.file_name();
                        let ok_filename = filename
                            .to_str()
                            .expect("invalid manifest name!");

                        let manifest_contents = std::fs::read_to_string(val.path()).unwrap();
                        let manifest_json: serde_json::Value =
                            serde_json::from_str(&manifest_contents).unwrap();

                        // println!("curently at manifes: {ok_filename}");
                        if ok_filename.ends_with(".json")
                            && (ok_filename.contains(query)
                                || parse_json_manifest::query_bin(&manifest_json, query).unwrap())
                        {
                            Some(Package {
                                bucket: "some".into(),
                                bucket_name: "some".into(),
                                name: ok_filename.into(),
                                version: parse_json_manifest::get_version(&manifest_json).unwrap(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vector<Package>>(),
            );
        }

        // for path in buckets{
        //     println!("{:#?}",path);
        // }
        //for each bucket find all manifests (bucketname/bucket/*)

        return Ok(out_vect);
    }
}
