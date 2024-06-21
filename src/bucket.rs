use crate::{package, parse_json_manifest, shell};
use druid::im::Vector;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum BucketError {
    #[error("Error: Thread paniced while searching for app! Paniced on error {0}")]
    ThreadSearchError(String),
}

pub fn clean_buckets() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = shell::run_cmd_in_depy_dir("scoop bucket rm *") {
        log::error!("Couldn't clean buckets!\nError:{err}");
        return Err(Box::new(shell::ShellError::CleanBucketError));
    };
    Ok(())
}

/// Adds a bucket to the depy/scoop instalation
pub fn add_bucket(
    bucket_url: &str,
    bucket_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Adding bucket: {bucket_name}");
    let cmd_output = match shell::run_cmd_in_depy_dir(&format!(
        "scoop bucket add {bucket_name} {bucket_url}"
    )) {
        Ok(out) => out,
        Err(err) => {
            log::error!("Failed to add bucket name: {bucket_name} bucket url: {bucket_url}.\nGot error{err}");
            return Err(Box::new(shell::ShellError::AddBucketError));
        }
    };

    if !cmd_output.contains(&format!("The {bucket_name} bucket was added successfully"))
        && !cmd_output.contains(&format!("The '{bucket_name}' bucket already exists"))
    {
        log::error!("Failed to add bucket name: {bucket_name} bucket url: {bucket_url},\nScoop output was:\n{cmd_output}");
        return Err(Box::new(shell::ShellError::AddBucketError));
    };
    Ok(())
}

pub fn resolve_bucket_name(bucket_name: &str) -> &str {
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

pub fn resolve_bucket_raw(bucket_name: &str) -> String {
    let bucket_url = resolve_bucket_name(bucket_name);
    parse_github_to_raw(bucket_url)
}

fn query_single_bucket(
    query: &str,
    bucket: std::path::PathBuf,
    deep_search: bool,
) -> Result<Vector<package::Package>, Box<dyn std::error::Error>> {
    let manifests = std::fs::read_dir(bucket.join("bucket"));

    let manifests = match manifests {
        Ok(out) => out,
        Err(err) => {
            println!("Found inoutid bucket at:{:#?}", bucket);
            return Err(err.into());
        }
    };

    Ok(manifests
        //TODO error handling
        .filter_map(|out| out.ok())
        .filter_map(|out| {
            let filename = out.file_name();
            let filename = filename
                .to_str()
                .expect("invalid manifest name!");

            let manifest_contents = std::fs::read_to_string(out.path()).unwrap();
            //TODO error handling
            let manifest_json: serde_json::Value =
                serde_json::from_str(&manifest_contents).unwrap();

            // println!("{filename}");
            if filename.ends_with(".json")
                && (filename.contains(query)
                    || (deep_search
                        && parse_json_manifest::query_bin(&manifest_json, query).unwrap()))
            {
                //TODO proper bucket naming
                Some(package::Package {
                    bucket: "some".into(),
                    bucket_name: "some".into(),
                    name: filename.into(),
                    version: parse_json_manifest::get_version(&manifest_json).unwrap(),
                })
            } else {
                None
            }
        })
        .collect::<Vector<package::Package>>())
}

pub fn query_local_buckets(
    query: &str,
    deep_search: bool,
) -> Result<Vector<package::Package>, Box<dyn std::error::Error>> {
    let mut out_vect = Vector::<package::Package>::new();

    //TODO document this behaviour and error handling
    let buckets = std::fs::read_dir(crate::dir::get_depy_dir_location() + "\\buckets")
        .expect("Couldn't find depy installation");

    //TODO error handling
    for bucket in buckets {
        let bucket = match bucket {
            Ok(out) => out,
            Err(err) => return Err(err.into()),
        };

        //TODO error handling
        out_vect.extend(query_single_bucket(query, bucket.path(), deep_search).unwrap());
    }
    return Ok(out_vect);
}
