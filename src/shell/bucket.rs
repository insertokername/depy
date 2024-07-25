use super::run_cmd_in_depy_dir;
use crate::{dir, package, parsing, shell::error::ShellError};
use druid::im::Vector;
use std::os::windows::process::CommandExt;

/// Deletes all buckets installed in the depy/scoop instalation
pub fn clean_buckets() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = run_cmd_in_depy_dir("scoop bucket rm *") {
        return Err(Box::new(ShellError::CleanBucketError(err.to_string())));
    };
    Ok(())
}

/// Adds a bucket to the depy/scoop instalation
pub fn add_bucket(bucket_url: &str, bucket_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Adding bucket: '{bucket_name}' ...");
    let cmd_output =
        match run_cmd_in_depy_dir(&format!("scoop bucket add {bucket_name} {bucket_url}")) {
            Ok(out) => out,
            Err(err) => {
                return Err(Box::new(ShellError::AddBucketError(
                    bucket_name.to_string(),
                    bucket_url.to_string(),
                    err.to_string(),
                )));
            }
        };

    if !cmd_output.contains(&format!("The {bucket_name} bucket was added successfully"))
        && !cmd_output.contains(&format!("The '{bucket_name}' bucket already exists"))
    {
        return Err(Box::new(ShellError::AddBucketError(
            bucket_name.to_string(),
            bucket_url.to_string(),
            cmd_output,
        )));
    };
    Ok(())
}

/// Remove a bucket to the depy/scoop instalation
pub fn remove_bucket(bucket_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Removing bucket: '{bucket_name}' ...");
    let cmd_output = match run_cmd_in_depy_dir(&format!("scoop bucket rm {bucket_name}")) {
        Ok(out) => out,
        Err(err) => {
            return Err(Box::new(ShellError::RemoveBucketError(
                bucket_name.to_string(),
                err.to_string(),
            )));
        }
    };

    if !cmd_output.contains(&format!(
        "The {bucket_name} bucket was removed successfully"
    )) {
        return Err(Box::new(ShellError::RemoveBucketError(
            bucket_name.to_string(),
            cmd_output,
        )));
    };
    Ok(())
}

/// Returs the origin of the bucket (the url where it was installed from)
fn find_bucket_origin(bucket: &std::path::PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("cmd")
        .arg("/C")
        .arg(format!(
            "git -C {} config remote.origin.url",
            bucket.to_string_lossy()
        ))
        .creation_flags(0x08000000)
        .output()?;

    if !output.status.success() {
        // let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Box::new(ShellError::BucketUrlError(
            bucket.to_string_lossy().to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Returns Vec<(name, url)>
pub fn list_buckets() -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let buckets = std::fs::read_dir(dir::get_depy_dir_location() + "\\buckets").map_err(|err| {
        Box::new(ShellError::ReadError(
            dir::get_depy_dir_location() + "\\buckets",
            err.to_string(),
        ))
    })?;

    Ok(buckets
        .into_iter()
        .map(|bucket| {
            let bucket = bucket.map_err(|err| {
                ShellError::ReadError(
                    "a bucket in depy/scoop/buckets".to_string(),
                    err.to_string(),
                )
            })?;
            let url = find_bucket_origin(&bucket.path());
            match url {
                Ok(ok_url) => Ok((bucket.file_name().to_string_lossy().to_string(), ok_url)),
                Err(err) => Err(err),
            }
        })
        .collect::<Result<_, _>>()?)
}

/// Parses common bucket names to their respective url
fn parse_common_name(bucket_name: &str) -> &str {
    match bucket_name {
        "main" => "https://github.com/ScoopInstaller/Main",
        "extras" => "https://github.com/ScoopInstaller/Extras",
        "versions" => "https://github.com/ScoopInstaller/Versions",
        _ => bucket_name,
    }
}

/// Transforms a github url to a githubusercontent url
fn parse_github_url(bucket_url: &str) -> String {
    if !bucket_url.starts_with("https://github.com") {
        log::error!("Expected bucket: \"{bucket_url}\" to be github repository! ");
        std::process::exit(1)
    }
    bucket_url.replace("github.com", "raw.githubusercontent.com") + "/master/bucket"
}

/// Returns the url of a bucket in a githubusercontent raw form
pub fn parse_bucket(bucket_name: &str) -> String {
    let bucket_url = parse_common_name(bucket_name);
    parse_github_url(bucket_url)
}

/// Returns all packages that match a query
fn query_single_bucket(
    query: &str,
    bucket: std::path::PathBuf,
    deep_search: bool,
) -> Result<Vector<package::Package>, Box<dyn std::error::Error>> {
    let manifests = std::fs::read_dir(bucket.join("bucket")).map_err(|err| {
        ShellError::ReadError(bucket.to_string_lossy().to_string(), err.to_string())
    })?;

    let bucket_url = find_bucket_origin(&bucket)?;
    let bucket_name = bucket.file_name().unwrap().to_string_lossy().to_string();

    Ok(manifests
        .filter_map(|out| out.ok())
        .map(|out| {
            let filename = out.file_name().to_string_lossy().to_string();
            let no_prefix_filename = filename.strip_suffix(".json").unwrap_or(&filename);
            let file_path = out.path().to_string_lossy().to_string();

            let mut json_file = None;

            if no_prefix_filename.contains(query)
                || (deep_search && {
                    let temp = match parsing::parse_json::read_json_file(&file_path) {
                        Ok(ok) => ok,
                        Err(err) => {
                            log::debug!("Found an improperly formated package named: {file_path}\n Format error was: {}\n\nSkipping over it", err.to_string());
                            return Ok(None);
                        }
                    };
                    let result = parsing::parse_json::query_bin(&temp, query)?;
                    json_file = Some(temp);
                    result
                })
            {
                let temp: serde_json::Value= match json_file{
                    Some(some) => some,
                    None => match parsing::parse_json::read_json_file(&file_path) {
                        Ok(ok) => ok,
                        Err(err) => {
                            log::debug!("Found an improperly formated package named: {file_path}\n Format error was: {}\n\nSkipping over it", err.to_string());
                            return Ok(None);
                        }
                    }
                };

                let version=parsing::parse_json::get_version(&temp)
                    .map_err(|err| ShellError::ManifestParseError(filename.clone(), err.to_string()))?;

                Ok(Some(package::Package {
                    bucket_url: bucket_url.clone(),
                    bucket_name: bucket_name.clone(),
                    name: no_prefix_filename.to_string(),
                    version: version,
                }))
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vector<Option<package::Package>>, Box<dyn std::error::Error>>>()?
        .into_iter()
        .filter_map(|a| a)
        .collect())
}

/// Finds a package matching the query in any buckets in the depy/scoop/buckets
pub fn query_all_buckets(
    query: &str,
    deep_search: bool,
) -> Result<Vector<package::Package>, Box<dyn std::error::Error>> {
    let mut out_vect = Vector::<package::Package>::new();

    let depy_location = crate::dir::get_depy_dir_location();
    let buckets = std::fs::read_dir(depy_location.clone() + "\\buckets")
        .map_err(|err| ShellError::ReadError(depy_location, err.to_string()))?;

    for bucket in buckets {
        let bucket = match bucket {
            Ok(out) => out,
            Err(err) => return Err(err.into()),
        };

        out_vect.extend(query_single_bucket(query, bucket.path(), deep_search)?);
    }
    return Ok(out_vect);
}
