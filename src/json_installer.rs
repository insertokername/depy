#[derive(thiserror::Error, Debug, PartialEq)]
pub enum InstallerError {
    #[error("Error: Invalid install json format!")]
    JsonFormatError,
    #[error("Error: Invalid response recieved!")]
    ResponseError,
}

use crate::{
    dir::cleanup_shims,
    manifest::Manifest,
    package,
    shell::{add_bucket, clean_buckets, init_depy, install_cleanly, make_devshell},
};

fn common_bucket_names(bucket_name: &str) -> &str {
    match bucket_name {
        "main" => "https://github.com/ScoopInstaller/Main",
        "extras" => "https://github.com/ScoopInstaller/Extras",
        "versions" => "https://github.com/ScoopInstaller/Versions",
        _ => bucket_name,
    }
}

fn parse_github_to_raw(bucket_url: &str) -> String {
    if !bucket_url.starts_with("https://github.com") {
        println!("Expected bucket: \"{bucket_url}\" to be github repository! ");
        std::process::exit(1)
    }
    bucket_url.replace("github.com", "raw.githubusercontent.com") + "/master/bucket"
}

fn resolve_bucket(bucket_name: &str) -> String {
    let bucket_url = common_bucket_names(bucket_name);
    parse_github_to_raw(bucket_url)
}

/// Installs all programs specified in a json file
pub fn install(install_json: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    clean_buckets()?;
    init_depy()?;
    let mut manifest_vec: Vec<Manifest> = vec![];
    let packages = if let Some(out) = install_json.as_array() {
        out
    } else {
        log::error!("Invalid install json, expected the installer to be an array of packages!");
        return Err(Box::new(InstallerError::JsonFormatError));
    };
    for package in packages {
        let package = match package::Package::from_value(package) {
            Ok(out) => out,
            Err(err) => {
                log::error!("Encountered error {err} while parsing packages!");
                return Err(Box::new(err));
            }
        };

        let bucket_url = resolve_bucket(&package.bucket);
        let app_url = bucket_url + "/" + &package.name + ".json";

        let response = match ureq::get(&app_url).call() {
            Ok(out) => out,
            Err(err) => {
                log::error!("\n\nCould not get app '{}' from url:{app_url}, check the following:\n1. You are connected to the internet\n2. The package is correctly spelled / it exists\n3. If you are using a custom bucket check that there is a bucket dir and its on the master branch\n\n",package.name);
                log::debug!("Recieved error from github was:{err}");
                return Err(Box::new(InstallerError::ResponseError));
            }
        };

        let actual_manifest = match response.into_string() {
            Ok(out) => out,
            Err(err) => {
                log::error!("Got an invalid answer from github! {err}");
                return Err(Box::new(err));
            }
        };

        let parsed_manifest = match Manifest::from_str(
            &actual_manifest,
            package.name.to_string(),
            package.version.to_string(),
        ) {
            Ok(out) => out,
            Err(err) => {
                log::error!("Manifest body:{actual_manifest}\nGot an invalid manifest!");
                return Err(err);
            }
        };
        manifest_vec.push(parsed_manifest);

        add_bucket(common_bucket_names(&package.bucket), &package.bucket_name)?;
    }

    for man in &manifest_vec {
        install_cleanly(&man.name, &man.version)?;
    }

    cleanup_shims()?;
    make_devshell(manifest_vec)?;
    cleanup_shims()?;

    Ok(())
}
