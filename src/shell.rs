use crate::{dir, manifest};

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

pub fn make_devshell(manifests: Vec<manifest::Manifest>){
    for man in manifests{
        // first add all paths to the PATH
        // trb sa merem prin manifest la fiecare path si sa merem in insatll folder (depy/scoop/apps/name/version)
        // adaugam al o variabila numita temp_path
        // cand instantiem shell-ul prefixuim PATH-ul cu temp_path

        // add all shims to .localshims
        // add all required shims from the bin attr
        // clear .localshims if not cleared
        // move every content of the shim folder to .localshims
        // add .localshims to temp_path

        // set all envs
        // read every set env from the manifest and insert it into a map 
    }
}