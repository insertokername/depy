use crate::dir;

use std::io::{BufRead, Write};

/// updates scoop and creates depy directory if doesn't allready exist
pub fn init_depy() -> Result<(), Box<dyn std::error::Error>> {
    dir::init_depy_dir()?; // makes the %userprofile%/depy/scoop dirs if not allready existing
    let handle = duct::cmd!("cmd", "/C", "scoop update")
        .env("SCOOP", dir::get_depy_dir_location())
        .stderr_to_stdout()
        .reader()?;
    
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();

    let reader = std::io::BufReader::new(handle);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                writeln!(stdout_lock, "{}", line).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    Ok(())
}

/// enters a dev shell with all environment variables set
pub fn clean_install(
    app_name: &str,
    app_version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let handle = duct::cmd!(
        "cmd",
        "/C",
        [
            "scoop config use_isolated_path DEPY_TEMP_VAL & ",
            "scoop bucket add main & ",
            &format!("scoop install {app_name}@{app_version} & "),
            "set DEPY_TEMP_VAL= & ",
            "setx DEPY_TEMP_VAL %DEPY_TEMP_VAL% & ",
            "scoop config rm use_isolated_path"
        ]
        .concat()
    )
    .env("SCOOP", dir::get_depy_dir_location())
    .stderr_to_stdout()
    .reader()?;

    // provide real time rendering of the stdout of the command
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();

    let reader = std::io::BufReader::new(handle);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                writeln!(stdout_lock, "{}", line).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    Ok(())
}