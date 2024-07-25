use crate::{dir, shell::run_cmd_in_depy_dir};

use super::error::ShellError;

/// Purges each package that is installed in the depy/scoop directory
pub fn clean_depy_packages(force_uninstall: bool) -> Result<(), Box<dyn std::error::Error>> {
    let packages: Vec<String> = std::fs::read_dir(dir::get_depy_dir_location() + "\\apps")?
        .into_iter()
        .map(|file| match file {
            Ok(file) => Ok(file.file_name().to_string_lossy().to_string()),
            Err(err) => Err(ShellError::ReadError(
                "a file in the depy\\scoop\\apps\\ directory".to_string(),
                err.to_string(),
            )),
        })
        .collect::<Result<Vec<String>, _>>()?
        .into_iter()
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
                    return Err(ShellError::PackageUninstallError(package, err.to_string()).into());
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
                return Err(Box::new(ShellError::PackageUninstallError(
                    package, cmd_output,
                )));
            }
        }
    }

    Ok(())
}

/// First calls `clean_depy_packages` and then deletes the depy folder
pub fn uninstall_depy(force_uninstall: bool) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Uninstalling depy apps...");

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

/// Cleans up any residual values in the depy path
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
        return Err(Box::new(ShellError::CleanupError(cmd_output)));
    }

    log::info!("Cleaned up successfuly!");
    log::debug!("Command output:\n{cmd_output}");
    Ok(())
}
