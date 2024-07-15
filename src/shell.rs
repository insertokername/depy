use path_absolutize::Absolutize;

use crate::{dir, manifest};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ShellError {
    #[error("Error: Couldn't execute a command!\nGot error: {0}")]
    ExecutionError(String),
    #[error("Error: Couldn't initialize depy!\n Got error {0}\n\nPlease make sure scoop is installed on your system!")]
    InitializeError(String),
    #[error("Error: Couldn't update the scoop instalation inside depy!\nGot scoop output {0}\nMake sure scoop is installed and you are connected to the internet!")]
    UpdateError(String),
    #[error("Error: Couldn't install an application!\n{0}\n\nFailed to install {1}, scoop error above ^^^^^^^^^^^^^^^^\n\nMake sure all required buckets are installed!")]
    InstallError(String, String),
    #[error("Error: Couldn't clean buckets!")]
    CleanBucketError,
    #[error("Error: Couldn't add a bucket!")]
    AddBucketError,
    #[error("Error: Couldn't remove a bucket!")]
    RemoveBucketError,
    #[error("Error: Couldn't create .depyenv folder!\nGot error{0}")]
    CreateEnvError(String),
    #[error("Error: Couldn't delete a file or folder!\nGot error{0}")]
    DeleteError(String),
    #[error("Error: Couldn't write to a file!\nGot error{0}")]
    WriteError(String),
    #[error("Error: Couldn't uninstall {0}, error:{1}\nIf you want to try force uninstall run cli with the -g/-d and the -f flag")]
    PackageUninstallError(String, String),
    #[error("Error: Couldn't cleanup a packages!")]
    CleanupError,
    #[error("Error: Couldn't make shims for{0}!\nGot error:{1}")]
    MakeShimError(String, String),
    #[error("Error: Couldn't copy shims!\nGot error:{0}")]
    CopyShimError(String),
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
        return Err(Box::new(ShellError::ExecutionError(stderr.to_string())));
    }

    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}

/// updates scoop and creates depy directory if doesn't allready exist
pub fn init_depy() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing depy directory...");
    dir::init_depy_dir();

    let cmd_output = match run_cmd_in_depy_dir(
        "scoop bucket add main & scoop update & scoop config scoop_branch develop",
    ) {
        Ok(cmd_output) => cmd_output,
        Err(err) => {
            return Err(Box::new(ShellError::InitializeError(err.to_string())));
        }
    };

    if !cmd_output.contains("Scoop was updated successfully!") {
        return Err(Box::new(ShellError::UpdateError(cmd_output)));
    }

    dir::cleanup_shims()?;
    Ok(())
}

fn generate_install_script(
    indentifier: &str,
    version: &str,
) -> String {
    [
        "scoop config use_isolated_path DEPY_TEMP_VAL & ",
        &if version == "latest" {
            format!("scoop install {} & ", indentifier)
        } else {
            format!("scoop install {}@{} & ", indentifier, version)
        },
        "set DEPY_TEMP_VAL= & ",
        "setx DEPY_TEMP_VAL %DEPY_TEMP_VAL% & ",
        "scoop config rm use_isolated_path",
    ]
    .concat()
}

fn attempt_install(
    name: &str,
    indentifier: &str,
    version: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let install_script = generate_install_script(indentifier, version);
    log::debug!("installing package{name}, running command:\n{install_script}",);

    let cmd_result = run_cmd_in_depy_dir(&install_script);

    let cmd_output = match cmd_result {
        Ok(out) => out,
        Err(err) => {
            log::error!("Failed to install {}, error:{}", name, err.to_string());
            return Err(err);
        }
    };

    if !cmd_output.lines().any(|line| {
        line.contains(&format!("{}", name))
            && (line.contains("was installed successfully!")
                || line.contains("is already installed"))
    }) {
        return Err(Box::new(ShellError::InstallError(
            cmd_output,
            name.to_string(),
        )));
    }

    Ok(cmd_output)
}

/// installs a program in the depy dir without adding it to path
pub fn install_cleanly(manifest: &manifest::Manifest) -> Result<(), Box<dyn std::error::Error>> {
    log::info!(
        "Installing {}@{}\nPlease do not terminate process as to not risk PATH damages...",
        manifest.name,
        manifest.version
    );

    let command_output = match attempt_install(&manifest.name, &manifest.url, &manifest.version) {
        Ok(ok) => ok,
        Err(first_err) => {
            log::debug!(
                "Couldn't install from urlZ!!\nGot error: {first_err}\nAttempting local install..."
            );
            match attempt_install(&manifest.name, &manifest.name, &manifest.version) {
                Ok(ok) => ok,
                Err(err) => {
                    log::error!("Couldn't install app from local!\nGot error {err}\n\nWhile trying to install package from url got error:{first_err}");
                    return Err(err);
                }
            }
        }
    };

    log::info!("{} installed successfully!\n", manifest.name);
    log::debug!("Command output:\n{command_output}");
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
                return Err(Box::new(ShellError::DeleteError(err.to_string())));
            };
        } else {
            if let Err(err) = std::fs::remove_dir_all(depyvenv) {
                return Err(Box::new(ShellError::DeleteError(err.to_string())));
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
            return Err(Box::new(ShellError::MakeShimError(
                    man.name,
                    err.to_string(),
                )));
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
        return Err(Box::new(ShellError::CreateEnvError(err.to_string())));
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
        return Err(Box::new(ShellError::CopyShimError(err.to_string())));
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
        std::fs::remove_dir_all(&depyvenv)?; //we should have read/write privileges of that folder since we created it a few seconds ago
        return Err(Box::new(ShellError::WriteError(err.to_string())));
    };
    if let Err(err) = std::fs::write(ps_devshell_loc, &ps_env_vars) {
        std::fs::remove_dir_all(&depyvenv)?;
        return Err(Box::new(ShellError::WriteError(err.to_string())));
    };
    if let Err(err) = std::fs::write(bat_devshell_loc, &bat_env_vars) {
        std::fs::remove_dir_all(&depyvenv)?;
        return Err(Box::new(ShellError::WriteError(err.to_string())));
    };

    log::info!("Successfully created venv dir!");
    Ok(())
}

pub fn clean_depy_packages(force_uninstall: bool) -> Result<(), Box<dyn std::error::Error>> {
    let packages: Vec<String> = std::fs::read_dir(dir::get_depy_dir_location() + "\\apps")
        .unwrap()
        .into_iter()
        .filter_map(|file| match file {
            Ok(file) => Some(
                file.file_name()
                    .to_string_lossy()
                    .to_string(),
            ),
            Err(err) => {
                log::info!(
                    "Couldn't read a file inside the depy folder! Got the following error: {}",
                    err.to_string()
                );
                None
            }
        })
        .filter(|package| package.to_lowercase() != "scoop")
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
                if force_uninstall {
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
            if force_uninstall {
                run_cmd_in_depy_dir(&format!(
                    "rmdir /S /Q {}\\apps\\{package}",
                    dir::get_depy_dir_location()
                ))
                .expect(&format!(
                    "\nERROR: Couldn't force remove package: {package}\n"
                ));
            } else {
                return Err(Box::new(ShellError::PackageUninstallError(package, cmd_output)));
            }
        }
    }

    Ok(())
}

pub fn uninstall_depy(force_uninstall: bool) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Uninstalling depy apps...");

    // get all programs from scoop list
    // run uninstall for all of them, if one fails just rm -rf it
    clean_depy_packages(force_uninstall)?;

    log::info!("Deleting depy directory...");

    if let Err(err) =
        remove_dir_all::remove_dir_all(dir::get_depy_dir_location() + "\\..\\..\\depy")
    {
        log::error!("Couldn't delete the depy folder %userprofile%/depy\nError was:\n{err}");
        return Err(Box::new(err));
    }

    log::info!(
        "Deletion successfull! If you want to fully remove depy run:\nscoop uninstall depy\n"
    );

    Ok(())
}

pub fn cleanup_path() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Cleaning up any path aditions made by depy!");
    let cmd_output = match run_cmd_in_depy_dir(
        &[
            "scoop config use_isolated_path DEPY_TEMP_VAL & ",
            "set DEPY_TEMP_VAL= & ",
            "setx DEPY_TEMP_VAL %DEPY_TEMP_VAL% & ",
            "scoop config rm use_isolated_path",
        ]
        .concat(),
    ) {
        Ok(out) => out,
        Err(err) => {
            log::error!("Failed to cleanup paths, error:{err}");
            return Err(err);
        }
    };
    if !(cmd_output.contains("SUCCESS: Specified value was saved")
        && cmd_output.contains("'use_isolated_path' has been removed"))
    {
        log::error!("Scoop errored out on:\n{cmd_output}");
        log::error!("\n\nFailed to cleanup paths, output above ^^^^^^^^^^^^^^^^\n\n");
        return Err(Box::new(ShellError::CleanupError));
    }

    log::info!("Cleaned up successfuly!");
    log::debug!("Command output:\n{cmd_output}");
    Ok(())
}
