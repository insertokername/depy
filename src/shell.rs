fn get_depy_dir_location() -> String {
    let user_profile =
        std::env::var("USERPROFILE").expect("%USERPROFILE% environment variable not found");
    format!("{}\\depy\\scoop", user_profile)
}

fn init_depy_dir() -> Result<(), Box<dyn std::error::Error>> {
    let str_path = get_depy_dir_location() + "\\buckets";
    let dirpath = std::path::Path::new(&str_path);
    if !dirpath.exists() {
        std::fs::create_dir_all(&dirpath).expect("Failed to create directories");
    }
    Ok(())
}

/// enters a dev shell with all environment variables set
pub fn enter_shell() -> Result<String, Box<dyn std::error::Error>> {
    init_depy_dir()?; // makes the %userprofile%/depy/scoop dirs if not allready existing

    Ok(duct::cmd!("cmd", "/C", "scoop bucket add main")
        .env("SCOOP", get_depy_dir_location())
        .env(
            "shit",
            std::env::var("PATH").expect("PATH variable not set, this should always be set"),
        )
        .read()
        .unwrap())
    // Ok(duct::cmd!("cmd", "/C", "echo caca & echo oterh caca").read().unwrap())
}
