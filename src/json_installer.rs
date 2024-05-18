// #[derive(thiserror::Error, Debug, PartialEq)]
// pub enum ParseInstallerError {
//     #[error("Error: Invalid system arch!")]

// }
use crate::{
    dir::cleanup_shims,
    manifest::Manifest,
    shell::{add_bucket, clean_buckets, install_cleanly, make_devshell},
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
    // println!("{}",install_json.is_object());
    clean_buckets()?;
    let mut manifest_vec: Vec<Manifest> = vec![];
    for package in install_json
        .as_array()
        .expect("Invalid install json, expected the installer to be an array of packages!\n")
    {
        // println!("{:#?}",package);
        let package_obj = package
            .as_object()
            .expect("Invalid Package format, expected each package to be an object\n");

        let package_bucket = package_obj
            .get("bucket_url")
            .expect("expected bucket_url attribute in package")
            .as_str()
            .expect("expected bucket_url value to be string");

        let package_bucket_name = package_obj
            .get("bucket_name")
            .expect("expected bucket_name attribute in package")
            .as_str()
            .expect("expected bucket_name value to be string");

        let package_name = package_obj
            .get("name")
            .expect("expected name attribute in package")
            .as_str()
            .expect("expected name value to be string");

        let package_version = package_obj
            .get("version")
            .expect("expected version attribute in package")
            .as_str()
            .expect("expected version value to be string");

        let bucket_url = resolve_bucket(package_bucket);
        let app_url = bucket_url + "/" + package_name + ".json";

        let response = ureq::get(&app_url).call().expect("Could not get bucket, check the following:\n1. You are connected to the internet\n2. The package is correctly spelled / it exists\n3. If you are using a custom bucket check that there is a bucket dir and its on the master branch\n\n");

        let actual_manifest = response
            .into_string()
            .expect("Got an invalid answer from github!");

        let parsed_manifest = Manifest::from_str(
            &actual_manifest,
            package_name.to_string(),
            package_version.to_string(),
        )
        .expect("Got an invalid manifest!");
        manifest_vec.push(parsed_manifest);

        println!("Adding bucket {package_bucket}");
        add_bucket(common_bucket_names(package_bucket), package_bucket_name)?;
    }

    for man in &manifest_vec {
        install_cleanly(&man.name, &man.version)?;
    }

    cleanup_shims()?;
    make_devshell(manifest_vec)?;
    cleanup_shims()?;
    
    Ok(())
}
