use path_absolutize::Absolutize;

use crate::{
    dir::{self},
    manifest,
};

use std::io::{BufRead, Write};

fn run_cmd_in_depy_dir(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    dir::init_depy_dir()?; // makes the %userprofile%/depy/scoop dirs if not allready existing
    let handle = duct::cmd!("cmd", "/C", cmd)
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

/// updates scoop and creates depy directory if doesn't allready exist
pub fn init_depy() -> Result<(), Box<dyn std::error::Error>> {
    dir::init_depy_dir()?; // makes the %userprofile%/depy/scoop dirs if they dont allready exist
    run_cmd_in_depy_dir("scoop update")?;
    Ok(())
}

/// enters a dev shell with all environment variables set
pub fn install_cleanly(app_name: &str, app_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_cmd_in_depy_dir(
        &[
            "scoop config use_isolated_path DEPY_TEMP_VAL & ",
            &if app_version == "latest" {
                format!("scoop install {} & ", app_name)
            } else {
                format!("scoop install {}@{} & ", app_name, app_version)
            },
            // &format!("scoop install {app_name}@{app_version} & "),
            "set DEPY_TEMP_VAL= & ",
            "setx DEPY_TEMP_VAL %DEPY_TEMP_VAL% & ",
            "scoop config rm use_isolated_path",
        ]
        .concat(),
    )?;
    Ok(())
}

pub fn make_devshell(manifests: Vec<manifest::Manifest>) -> Result<(), Box<dyn std::error::Error>> {
    let depyvenv = std::path::Path::new("./.depyvenv");

    if depyvenv.exists() {
        if depyvenv.is_file() {
            std::fs::remove_file(depyvenv)
                .expect("Couldn't remove .depyvenv!\nChange write permisions!\n");
        } else {
            std::fs::remove_dir_all(depyvenv)
                .expect("Couldn't remove .depyvenv!\nChange write permisions!\n");
        }
    }

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
        run_cmd_in_depy_dir(
            &[
                "scoop config use_isolated_path DEPY_TEMP_VAL & ",
                &if man.version == "latest" {
                    format!("scoop reset {} & ", &man.name)
                } else {
                    format!("scoop reset {}@{} & ", &man.name, &man.version)
                },
                "set DEPY_TEMP_VAL= & ",
                "setx DEPY_TEMP_VAL %DEPY_TEMP_VAL% & ",
                "scoop config rm use_isolated_path",
            ]
            .concat(),
        )?;

        // set all envs
        for var in man.env_vars {
            let formated_val = dir::expand_vars(&var.value, &man.name, &man.version);
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

    std::fs::create_dir(depyvenv).expect("Couldn't create .depyvenv!\nChange write permisions!\n");

    // move every content of the shim folder to .localshims
    // add .localshims to temp_path
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    let source_shims = [&dir::get_depy_dir_location(), "\\shims"].concat();
    let local_shims = "./.depyvenv/localshims";

    fs_extra::dir::copy(source_shims, local_shims, &options).expect("couldn't copy shims folder\n");

    let path_local_shims = std::path::Path::new(local_shims);
    paths += &[
        path_local_shims.absolutize().unwrap().to_str().unwrap(),
        ";",
    ]
    .concat();

    let empty_devshell_loc = std::path::Path::new("./.depyvenv/activate");
    let bat_devshell_loc = std::path::Path::new("./.depyvenv/activate.bat");
    let ps_devshell_loc = std::path::Path::new("./.depyvenv/activate.ps1");

    bat_env_vars += &["set PATH=", &paths, "%PATH%\n"].concat();
    ps_env_vars += &["$env:PATH = \"", &paths, "\" + $env:PATH\n"].concat();

    std::fs::write(empty_devshell_loc, "").expect(&format!(
        "Couldn't write devshell!\nChange write permissions!\n"
    ));
    std::fs::write(ps_devshell_loc, &ps_env_vars).expect(&format!(
        "Couldn't write devshell!\nChange write permissions!\n"
    ));
    std::fs::write(bat_devshell_loc, &bat_env_vars).expect(&format!(
        "Couldn't write devshell!\nChange write permissions!\n"
    ));

    Ok(())
}

pub fn clean_buckets()->Result<(), Box<dyn std::error::Error>> {
    run_cmd_in_depy_dir("scoop bucket rm *")?;
    Ok(())
}

pub fn add_bucket(bucket_url: &str, bucket_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_cmd_in_depy_dir(&format!("scoop bucket add {bucket_name} {bucket_url}"))?;
    Ok(())
}
