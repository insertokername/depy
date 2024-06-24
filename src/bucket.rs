use std::{ffi::OsString, fs::DirEntry, path::PathBuf};

use crate::{package, parse_json_manifest, shell};
use druid::im::Vector;

#[derive(thiserror::Error, Debug)]
pub enum BucketError {
    #[error("Error: Thread paniced while searching for app! Paniced on error {0}")]
    ThreadSearchError(String),
    #[error("Error: Couldn't determine the url of bucket {0}!")]
    BucketUrlError(PathBuf),
    #[error("Error: Couldn't determine the name of a file {0:?}!")]
    FileNameError(OsString),
    #[error("Error: Couldn't read manifest: {0}!\nGot error: {1}")]
    ManifestReadError(String, #[source] std::io::Error),
    #[error("Error: Couldn't parse manifest: {0}!\nGot error: {1}")]
    ManifestParseError(String, #[source] Box<dyn std::error::Error>),
    #[error("Error: Couldn't open the depy instalation folder: {0}!\nGot the following error: {1}\nMake sure that you have followed all depy install instructions and that the folder permisions are correct!")]
    DepyInstallationError(String, #[source] std::io::Error),
    
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
        log::error!("Expected bucket: \"{bucket_url}\" to be github repository! ");
        std::process::exit(1)
    }
    bucket_url.replace("github.com", "raw.githubusercontent.com") + "/master/bucket"
}

pub fn resolve_bucket_raw(bucket_name: &str) -> String {
    let bucket_url = resolve_bucket_name(bucket_name);
    parse_github_to_raw(bucket_url)
}

fn open_json_file_helper(dir_entry: &DirEntry, filename: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>>  {
    let manifest_contents = std::fs::read_to_string(dir_entry.path()).map_err(|e: std::io::Error| {
        Box::new(BucketError::ManifestReadError(
            filename.to_string(),
            e,
        ))
    })?;

    let manifest_json: serde_json::Value =
        serde_json::from_str(&manifest_contents).map_err(|e| {
           Box::new(BucketError::ManifestParseError(
                filename.to_string(),
                Box::new(e),
            ))
        })?;

    Ok(manifest_json)
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
        .filter_map(|out| out.ok())
        .map(|out| {
            let filename = out.file_name();
            let filename = filename
                .to_str()
                .ok_or_else(|| Box::new(BucketError::FileNameError(filename.clone())))?;

            let mut json_file  = None;
            
            if filename.ends_with(".json")
                && (filename.contains(query)
                    || (deep_search
                        && {
                            let temp = open_json_file_helper(&out, &filename)?;
                            let result = parse_json_manifest::query_bin(&temp, query)?;
                            json_file = Some(temp);
                            result
                        }))
            {
                
                if json_file.is_none(){
                    json_file = Some(open_json_file_helper(&out, &filename)?);
                }
                Ok(Some(package::Package {
                    // this will never be used in the context of a package search since you have to query local buckets you don t need their url
                    bucket: "".to_string(),
                    bucket_name: bucket
                        .file_name()
                        .ok_or_else(|| Box::new(BucketError::FileNameError(bucket.clone().into_os_string())))?
                        .to_str()
                        .ok_or_else(|| Box::new(BucketError::FileNameError(bucket.clone().into_os_string())))?
                        .to_string(),
                    name: filename.to_string(),
                    // unwrap here is safe since it is guarateed to be Some by the assignment a few lines above
                    version: parse_json_manifest::get_version(&json_file.unwrap()).map_err(|e|BucketError::ManifestParseError(filename.to_string(), e))?,
                }))
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vector<Option<package::Package>>,Box<dyn std::error::Error>>>()?
        .into_iter()
        .filter_map(|a|a)
        .collect())
}

pub fn query_local_buckets(
    query: &str,
    deep_search: bool,
) -> Result<Vector<package::Package>, Box<dyn std::error::Error>> {
    let mut out_vect = Vector::<package::Package>::new();

    let depy_location = crate::dir::get_depy_dir_location();
    let buckets = std::fs::read_dir(depy_location.clone()+"\\buckets")
        .map_err(|e|BucketError::DepyInstallationError(depy_location, e))?;

    for bucket in buckets {
        let bucket = match bucket {
            Ok(out) => out,
            Err(err) => return Err(err.into()),
        };

        out_vect.extend(query_single_bucket(query, bucket.path(), deep_search)?);
    }
    return Ok(out_vect);
}
