use crate::{
    dir::{self, expand_vars, get_depy_dir_location},
    manifest,
};

use std::{
    io::{BufRead, Write},
    path,
};

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
pub fn clean_install(app_name: &str, app_version: &str) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn make_devshell(manifests: Vec<manifest::Manifest>) {
    let depyvenv = path::Path::new("./.depyvenv");

    if depyvenv.exists() {
        if depyvenv.is_file() {
            std::fs::remove_file(depyvenv)
                .expect("Couldn't remove .depyvenv!\nChange write permisions!\n");
        } else {
            std::fs::remove_dir_all(depyvenv)
                .expect("Couldn't remove .depyvenv!\nChange write permisions!\n");
        }
    }

    std::fs::create_dir(depyvenv).expect("Couldn't create .depyvenv!\nChange write permisions!\n");

    let empty_devshell_loc = path::Path::new("./.depyvenv/activate");
    let bat_devshell_loc = path::Path::new("./.depyvenv/activate.bat");
    let ps_devshell_loc = path::Path::new("./.depyvenv/activate.ps1");

    let mut ps_env_vars = r###"function global:prompt { return "(CURENTLY IN DEV SHELL) " + (Get-Location) + "> " } $function:prompt = $function:prompt"###.to_string();
    ps_env_vars += "\n";
    let mut bat_env_vars = "@echo off & PROMPT (CURENTLY IN DEV SHELL) $P$G \n".to_string();

    let mut paths = "".to_string();

    for man in manifests {
        // first add all paths to the PATH
        // trb sa merem prin manifest la fiecare path si sa merem in insatll folder (depy/scoop/apps/name/version)
        // adaugam al o variabila numita temp_path
        // cand instantiem shell-ul prefixuim PATH-ul cu temp_path
        for path in man.added_paths {
            paths += &[
                &dir::get_version_location(&man.name, &man.version),
                "\\",
                &path.path,
                ";",
            ]
            .concat();
        }

        // add all shims to .localshims
        // add all required shims from the bin attr
        // clear .localshims if not cleared
        // move every content of the shim folder to .localshims
        // add .localshims to temp_path

        // set all envs
        for var in man.env_vars {
            let formated_val = expand_vars(&var.value, &man.name, &man.version);
            bat_env_vars += &["set \"", &var.name, "=", &formated_val, "\"\n"].concat();
            ps_env_vars += &[
                "Set-Item -Path Env:'",
                &var.name,
                "' -Value '",
                &formated_val,
                "'\n",
            ]
            .concat();
        }
    }

    bat_env_vars += &["set PATH=", &paths, "%PATH%"].concat();
    ps_env_vars += &["$env:PATH = \"", &paths, "\" + $env:PATH"].concat();
    //$env:PATH = "C:\New\Path;" + $env:PATH
    std::fs::write(empty_devshell_loc, "").expect(&format!(
        "Couldn't write devshell!\nChange write permissions!\n"
    ));
    std::fs::write(ps_devshell_loc, &ps_env_vars).expect(&format!(
        "Couldn't write devshell!\nChange write permissions!\n"
    ));
    std::fs::write(bat_devshell_loc, &bat_env_vars).expect(&format!(
        "Couldn't write devshell!\nChange write permissions!\n"
    ));
}
