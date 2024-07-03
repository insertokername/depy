#[derive(thiserror::Error, Debug, PartialEq)]
pub enum InstallerError {
    #[error("Error: Invalid install json format!")]
    JsonFormatError,
    #[error("Error: Invalid response recieved!")]
    ResponseError,
}

use crate::{bucket, dir, manifest::Manifest, package, shell};

/// Installs all programs specified in a json file
pub fn install(packages: &Vec<package::Package>) -> Result<(), Box<dyn std::error::Error>> {
    let mut manifest_vec: Vec<Manifest> = vec![];

    for package in packages {
        let bucket_url = bucket::resolve_bucket_raw(&package.bucket_url);
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
            app_url.clone(),
            package.version.to_string(),
        ) {
            Ok(out) => out,
            Err(err) => {
                log::error!("Manifest body:{actual_manifest}\nGot an invalid manifest!");
                return Err(err);
            }
        };
        manifest_vec.push(parsed_manifest);
    }

    for man in &manifest_vec {
        shell::install_cleanly(&man)?;
    }

    dir::cleanup_shims()?;
    shell::make_devshell(manifest_vec)?;
    dir::cleanup_shims()?;

    Ok(())
}
