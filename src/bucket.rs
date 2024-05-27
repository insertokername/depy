use crate::shell::{ShellError, run_cmd_in_depy_dir};

pub fn clean_buckets() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = run_cmd_in_depy_dir("scoop bucket rm *") {
        log::error!("Couldn't clean buckets!\nError:{err}");
        return Err(Box::new(ShellError::CleanBucketError));
    };
    Ok(())
}

/// Adds a bucket to the depy/scoop instalation
pub fn add_bucket(bucket_url: &str, bucket_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Adding bucket: {bucket_name}");
    let cmd_output = match run_cmd_in_depy_dir(&format!(
        "scoop bucket add {bucket_name} {bucket_url}"
    )) {
        Ok(out) => out,
        Err(err) => {
            log::error!("Failed to add bucket name: {bucket_name} bucket url: {bucket_url}.\nGot error{err}");
            return Err(Box::new(ShellError::AddBucketError));
        }
    };

    if !cmd_output.contains(&format!("The {bucket_name} bucket was added successfully"))
        && !cmd_output.contains(&format!("The '{bucket_name}' bucket already exists"))
    {
        log::error!("Failed to add bucket name: {bucket_name} bucket url: {bucket_url},\nScoop output was:\n{cmd_output}");
        return Err(Box::new(ShellError::AddBucketError));
    };
    Ok(())
}