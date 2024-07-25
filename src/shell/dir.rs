use crate::shell::error::ShellError;

/// Returns the userprofile env variable
fn get_user_profile() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(out) = std::env::var("USERPROFILE") {
        Ok(out)
    } else {
        // log::error!("%USERPROFILE% environment variable not found");
        // std::process::exit(1);
        return Err(Box::new(ShellError::MissingUserProfileError));
    }
}

/// Returns the location of the depy installation  directory
pub fn get_depy_scoop_location() -> Result<String, Box<dyn std::error::Error>> {
    let user_profile = get_user_profile()?;
    Ok(format!("{}\\depy\\scoop", user_profile))
}

/// Returns the location of the main scoop installation directory
pub fn get_scoop_dir_location() -> Result<String, Box<dyn std::error::Error>> {
    let user_profile = get_user_profile()?;
    Ok(format!("{}\\scoop", user_profile))
}

/// Deletes everything inside a directory
pub fn clear_directory<P: AsRef<std::path::Path>>(
    dir: P,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            std::fs::remove_file(path)?;
        } else if path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        }
    }
    Ok(())
}

/// Returns the location of a specific version of an app
pub fn get_version_location(
    name: &str,
    version: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok([
        &(get_depy_scoop_location()?),
        "\\apps\\",
        name,
        "\\",
        version,
    ]
    .concat())
}

/// Creates all directories necessary for depy to function under the %userprofile%/depy directory
pub fn init_depy_dir() -> Result<(), Box<dyn std::error::Error>> {
    let str_path = get_depy_scoop_location()?;
    let str_bucketpath = [&str_path, "\\buckets"].concat();
    let bucketpath = std::path::Path::new(&str_bucketpath);
    if !bucketpath.exists() {
        if let Err(err) = std::fs::create_dir_all(&bucketpath) {
            return Err(Box::new(ShellError::CreateFolderError(
                bucketpath.to_string_lossy().to_string(),
                err.to_string(),
            )));
        }
    }

    let str_shimpath = [&str_path, "\\shims"].concat();
    let shimpath = std::path::Path::new(&str_shimpath);
    if !shimpath.exists() {
        if let Err(err) = std::fs::create_dir_all(&shimpath) {
            return Err(Box::new(ShellError::CreateFolderError(
                shimpath.to_string_lossy().to_string(),
                err.to_string(),
            )));
        }
    }

    let str_apppath = [&str_path, "\\apps"].concat();
    let apppath = std::path::Path::new(&str_apppath);
    if !apppath.exists() {
        if let Err(err) = std::fs::create_dir_all(&apppath) {
            return Err(Box::new(ShellError::CreateFolderError(
                apppath.to_string_lossy().to_string(),
                err.to_string(),
            )));
        }
    }

    let str_scooplocation = get_scoop_dir_location()? + "\\apps\\scoop";
    let str_depy_scooplocation = str_apppath + "\\scoop";
    let scooplocation = std::path::Path::new(&str_scooplocation);
    let depy_scooplocation = std::path::Path::new(&str_depy_scooplocation);
    if !depy_scooplocation.exists() {
        if let Err(err) = copy_dir::copy_dir(&scooplocation, &depy_scooplocation) {
            return Err(Box::new(ShellError::CreateFolderError(
                depy_scooplocation.to_string_lossy().to_string(),
                err.to_string(),
            )));
        }
    };
    Ok(())
}
