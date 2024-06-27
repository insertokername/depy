use path_absolutize::Absolutize;

use crate::{dir, manifest};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ShellError {
    #[error("Error: Couldn't execute a command!")]
    ExecutionError,
    #[error("Error: Couldn't update the scoop instalation inside depy!")]
    UpdateError,
    #[error("Error: Couldn't install an application!")]
    InstallError,
    #[error("Error: Couldn't clean buckets!")]
    CleanBucketError,
    #[error("Error: Couldn't add a bucket!")]
    AddBucketError,
    #[error("Error: Couldn't create a file or folder!")]
    CreateError,
    #[error("Error: Couldn't delete a file or folder!")]
    DeleteError,
    #[error("Error: Couldn't write to a file!")]
    WriteError,
    #[error("Error: Couldn't uninstall a package!")]
    PackageUninstallError,
}

/// runs generic command inside the depy/scoop folder
pub fn run_cmd_in_depy_dir(cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("cmd")
        .arg("/C")
        .arg(cmd)
        .env("SCOOP", dir::get_depy_dir_location())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("Command failed with error: {stderr}");
        return Err(Box::new(ShellError::ExecutionError));
    }

    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}

/// updates scoop and creates depy directory if doesn't allready exist
pub fn init_depy() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing depy directory...");
    dir::init_depy_dir();

    let cmd_output =
        match run_cmd_in_depy_dir("scoop bucket add main & scoop update & scoop config scoop_branch develop") {
            Ok(cmd_output) => cmd_output,
            Err(err) => {
                log::error!(
                "Failed to run update command! Please make sure scoop is installed on your system!"
            );
                return Err(err);
            }
        };

    if !cmd_output.contains("Scoop was updated successfully!") {
        log::error!("Couldn't update scoop! Command output: {cmd_output}");
        return Err(Box::new(ShellError::UpdateError));
    }

    dir::cleanup_shims()?;
    Ok(())
}

/// installs a program in the depy dir without adding it to path
pub fn install_cleanly(
    manifest: &manifest::Manifest
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Installing {}@{}\nPlease do not terminate process as to not risk PATH damages...", manifest.name, manifest.version );
    let cmd_output = match run_cmd_in_depy_dir(
        &[
            "scoop config use_isolated_path DEPY_TEMP_VAL & ",
            &if manifest.version == "latest" {
                format!("scoop install {} & ", manifest.url)
            } else {
                format!("scoop install {}@{} & ", manifest.name, manifest.version)
            },
            "set DEPY_TEMP_VAL= & ",
            "setx DEPY_TEMP_VAL %DEPY_TEMP_VAL% & ",
            "scoop config rm use_isolated_path",
        ]
        .concat(),
    ) {
        Ok(out) => out,
        Err(err) => {
            log::error!("Failed to install {}, error:{err}",manifest.name);
            return Err(err);
        }
    };

    if !cmd_output.lines().any(|line| {
        line.contains(&format!("{}",manifest.name))
            && (line.contains("was installed successfully!")
                || line.contains("is already installed"))
    }) {
        log::error!("Scoop errored out on:\n{cmd_output}");
        log::error!("\n\nFailed to install {}, scoop error above ^^^^^^^^^^^^^^^^\n\n",manifest.name);
        return Err(Box::new(ShellError::InstallError));
    }

    log::info!("{} installed successfully!\n",manifest.name);
    log::debug!("Command output:\n{cmd_output}");
    Ok(())
}

/// Creates .depyenv folder in curent folder, containing activation scripts to temporarily add programs to the path
///
/// # IMPORTANT:
/// **This function assumes that packages are allready installed in your depy installation (%userprofile%/depy/scoop) please make sure to `install_cleanly` the app before running this**
pub fn make_devshell(manifests: Vec<manifest::Manifest>) -> Result<(), Box<dyn std::error::Error>> {
    let depyvenv = std::path::Path::new("./.depyvenv");

    if depyvenv.exists() {
        if depyvenv.is_file() {
            if let Err(err) = std::fs::remove_file(depyvenv) {
                log::error!("Couldn't remove .depyvenv!\n Got error {err}");
                return Err(Box::new(ShellError::DeleteError));
            };
        } else {
            if let Err(err) = std::fs::remove_dir_all(depyvenv) {
                log::error!("Couldn't remove .depyvenv!\n Got error {err}");
                return Err(Box::new(ShellError::DeleteError));
            };
        }
    };

    let mut ps_env_vars = r###"function global:prompt { return "(CURENTLY IN DEV SHELL) " + (Get-Location) + "> " } $function:prompt = $function:prompt"###.to_string();
    ps_env_vars += "\n";
    let mut bat_env_vars = "@echo off & PROMPT (CURENTLY IN DEV SHELL) $P$G \n".to_string();

    let mut paths = "".to_string();

    for man in manifests {
        for path in man.added_paths {
            paths += &[
                &dir::get_version_location(&man.name, &man.version),
                "\\",
                &path,
                ";",
            ]
            .concat();
        }

        // add all required shims from the bin attr
        log::info!("Adding shims for {}", &man.name);
        let cmd_out = match run_cmd_in_depy_dir(
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
        ) {
            Ok(out) => out,
            Err(err) => {
                log::error!("Failed to make shims for {}, error:{err}", &man.name);
                return Err(err);
            }
        };
        log::debug!("Shim making output for {}:\n {cmd_out}", &man.name);

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

    if let Err(err) = std::fs::create_dir(depyvenv) {
        log::error!("Couldn't create .depyvenv!\nGot error: {err}");
        return Err(Box::new(ShellError::CreateError));
    };

    // move every content of the shim folder to .localshims
    // add .localshims to temp_path
    log::info!("Creating venv dir...");

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    let source_shims = [&dir::get_depy_dir_location(), "\\shims"].concat();
    let local_shims = "./.depyvenv/localshims";

    if let Err(err) = fs_extra::dir::copy(source_shims, local_shims, &options) {
        log::error!("Failed to copy shims to {local_shims}\nGot error: {err}");
        return Err(Box::new(err));
    };

    let path_local_shims = std::path::Path::new(local_shims);
    paths += &[
        path_local_shims
            .absolutize()
            .unwrap()
            .to_str()
            .unwrap(),
        ";",
    ]
    .concat();

    let empty_devshell_loc = std::path::Path::new("./.depyvenv/activate");
    let bat_devshell_loc = std::path::Path::new("./.depyvenv/activate.bat");
    let ps_devshell_loc = std::path::Path::new("./.depyvenv/activate.ps1");

    bat_env_vars += &["set PATH=", &paths, "%PATH%\n"].concat();
    ps_env_vars += &["$env:PATH = \"", &paths, "\" + $env:PATH\n"].concat();

    if let Err(err) = std::fs::write(empty_devshell_loc, "") {
        log::error!("Couldn't write devshell!\nGot error:{err}");
        std::fs::remove_dir_all(&depyvenv)?; //we should have read/write privileges of that folder since we created it a few seconds ago
        return Err(Box::new(ShellError::WriteError));
    };
    if let Err(err) = std::fs::write(ps_devshell_loc, &ps_env_vars) {
        log::error!("Couldn't write devshell!\nGot error:{err}");
        std::fs::remove_dir_all(&depyvenv)?;
        return Err(Box::new(ShellError::WriteError));
    };
    if let Err(err) = std::fs::write(bat_devshell_loc, &bat_env_vars) {
        log::error!("Couldn't write devshell!\nGot error:{err}");
        std::fs::remove_dir_all(&depyvenv)?;
        return Err(Box::new(ShellError::WriteError));
    };

    log::info!("Successfully created venv dir!");
    Ok(())
}

pub fn clean_depy_packages()-> Result<(), Box<dyn std::error::Error>>{
        let cmd_output = match run_cmd_in_depy_dir(&format!("scoop list")) {
        Ok(out) => out,
        Err(err) => {
            log::error!("List packages.\nGot error{err}");
            return Err(Box::new(ShellError::AddBucketError));
        }
    };

    let mut iter = cmd_output.lines().peekable();

    while let Some(line) = iter.next() {
        if line.starts_with("Name") {
            iter.next();
            break;
        }
    }

    let packages: Vec<&str> = iter
        .filter_map(|line| line.split(' ').next())
        .filter(|package| !package.is_empty())
        .collect();

    for package in packages {
        log::info!("Uninstalling {package}");
        let cmd_output = match run_cmd_in_depy_dir(
            &[
                "scoop config use_isolated_path DEPY_TEMP_VAL & ",
                &format!("scoop uninstall {package} --purge & "),
                "set DEPY_TEMP_VAL= & ",
                "setx DEPY_TEMP_VAL %DEPY_TEMP_VAL% & ",
                "scoop config rm use_isolated_path",
            ]
            .concat(),
        ) {
            Ok(out) => out,
            Err(err) => {
                if crate::ARGS.force_uninstall {
                    run_cmd_in_depy_dir(&format!(
                        "rmdir /S /Q {}\\apps\\{package}",
                        dir::get_depy_dir_location()
                    ))
                    .expect(&format!(
                        "\nERROR: Couldn't force remove package: {package}\n"
                    ));
                    format!("'{package}' was uninstalled.")
                } else {
                    log::error!("Failed to uninstall {package}, error:{err}\nIf you want to try force uninstall set -f flag\n\n");
                    return Err(err);
                }
            }
        };

        if !cmd_output.contains(&format!("'{package}' was uninstalled"))
            && !cmd_output.contains(&format!("'{package}' isn't installed."))
        {
            if crate::ARGS.force_uninstall {
                run_cmd_in_depy_dir(&format!(
                    "rmdir /S /Q {}\\apps\\{package}",
                    dir::get_depy_dir_location()
                ))
                .expect(&format!(
                    "\nERROR: Couldn't force remove package: {package}\n"
                ));
            } else {
                log::error!("Failed to uninstall {package}, error:{cmd_output}\nIf you want to try force uninstall set -f flag\n\n");
                return Err(Box::new(ShellError::PackageUninstallError));
            }
        }
    }

    Ok(())
}

pub fn uninstall_depy() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Uninstalling depy apps...");

    // get all programs from scoop list
    // run uninstall for all of them, if one fails just rm -rf it
    clean_depy_packages()?;

    log::info!("Deleting depy directory...");

    if let Err(err) = remove_dir_all::remove_dir_all(dir::get_depy_dir_location()+"\\..\\..\\depy") {
        log::error!("Couldn't delete the depy folder %userprofile%/depy\nError was:\n{err}");
        return Err(Box::new(err));
    }

    log::info!(
        "Deletion successfull! If you want to fully remove depy run:\nscoop uninstall depy\n"
    );

    Ok(())
}
