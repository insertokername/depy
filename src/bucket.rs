use crate::{package, parse_json_manifest, shell};
use druid::im::Vector;

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

pub fn query_local_buckets(
    query: &str
) -> Result<Vector<package::Package>, Box<dyn std::error::Error>> {
    let mut out_vect = Vector::<package::Package>::new();

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

                    if ok_filename.ends_with(".json")
                        && (ok_filename.contains(query)
                            || parse_json_manifest::query_bin(&manifest_json, query).unwrap())
                    {
                        Some(package::Package {
                            bucket: "some".into(),
                            bucket_name: "some".into(),
                            name: ok_filename.into(),
                            version: parse_json_manifest::get_version(&manifest_json).unwrap(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vector<package::Package>>(),
        );
    }

    // for path in buckets{
    //     println!("{:#?}",path);
    // }
    //for each bucket find all manifests (bucketname/bucket/*)

    return Ok(out_vect);
}
