use crate::{
    parsing,
    shell::{dir, error::ShellError, run_cmd_in_depy_dir},
};
use path_absolutize::Absolutize;

/// Creates the .depyenv folder in curent folder, containing activation scripts to temporarily add programs to the path
///
/// # IMPORTANT:
/// **This function assumes that packages are allready installed in your depy installation (%userprofile%/depy/scoop) please make sure to `install_cleanly` the app before running this**
pub fn make_venv(manifests: Vec<parsing::Manifest>) -> Result<(), Box<dyn std::error::Error>> {
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
                &dir::get_version_location(&man.name, &man.version)?,
                "\\",
                &path,
                ";",
            ]
            .concat();
        }

        // add all required shims
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
            let formated_val = parsing::parse_json::expand_vars(&var.value, &man.name, &man.version)?;
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

    let source_shims = [&dir::get_depy_scoop_location()?, "\\shims"].concat();
    let local_shims = "./.depyvenv/localshims";

    if let Err(err) = fs_extra::dir::copy(source_shims, local_shims, &options) {
        return Err(Box::new(ShellError::CopyShimError(err.to_string())));
    };

    let path_local_shims = std::path::Path::new(local_shims);
    paths += &[
        path_local_shims.absolutize().unwrap().to_string_lossy().to_string().as_str(),
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
