use super::{bucket, cleanup, dir, error::ShellError, run_cmd_in_depy_dir, venv};
use crate::{package, parsing};

/// Used to generate a script to install a single app
fn generate_install_script(indentifier: &str, version: &str) -> String {
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

/// Updates scoop and creates depy directory if doesn't allready exist
pub fn init_depy() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing depy directory...");
    dir::init_depy_dir();

    let cmd_output = match run_cmd_in_depy_dir(
        "scoop bucket add main & scoop bucket add versions & scoop bucket add extras & scoop update & scoop config scoop_branch develop",
    ) {
        Ok(cmd_output) => cmd_output,
        Err(err) => {
            return Err(Box::new(ShellError::InitializeError(err.to_string())));
        }
    };

    if !cmd_output.contains("Scoop was updated successfully!") {
        return Err(Box::new(ShellError::UpdateError(cmd_output)));
    }

    cleanup::cleanup_shims()?;
    Ok(())
}

/// Tries to install an app from a single source (usually a url or from a local bucket)
fn attempt_install(
    name: &str,
    indentifier: &str,
    version: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let install_script = generate_install_script(indentifier, version);
    log::debug!("installing package{name}, running command:\n{install_script}",);

    let cmd_output = run_cmd_in_depy_dir(&install_script)?;

    if !cmd_output.lines().any(|line| {
        line.contains(&format!("{}", name))
            && (line.contains("was installed successfully!")
                || line.contains("is already installed"))
    }) {
        return Err(Box::new(ShellError::SingleInstallError(
            cmd_output,
            name.to_string(),
        )));
    }

    Ok(cmd_output)
}

/// Installs a program in the depy dir without adding it to path
/// First tries to install from the url form (`scoop install https://raw.githubusercontent.com/ScoopInstaller/Main/master/bucket/neovim.json@0.9.0`)
/// And if that errors out tries install from a local bucket (`scoop install neovim@0.9.0`)
pub fn install_cleanly(manifest: &parsing::Manifest) -> Result<(), Box<dyn std::error::Error>> {
    log::info!(
        "Installing {}@{}\nPlease do not terminate process as to not risk PATH damages...",
        manifest.name,
        manifest.version
    );

    let command_output = match attempt_install(&manifest.name, &manifest.url, &manifest.version) {
        Ok(ok) => ok,
        Err(first_err) => {
            log::debug!(
                "Couldn't install from url!!\nGot error: {first_err}\nAttempting local install..."
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

/// Installs a list of packages
pub fn install(mut packages: Vec<package::Package>) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the package vector contains two of the same package
    packages.sort();
    packages.dedup_by(|first, second| {
        if (*first).eq(second) {
            log::info!(
                "Only installing first of the duplicate packages:\nfirst: {:#?}\nsecond: {:#?}",
                first,
                second
            );
            true
        } else {
            false
        }
    });

    let mut manifest_vec: Vec<parsing::Manifest> = vec![];

    for package in packages {
        let bucket_url = bucket::parse_bucket(&package.bucket_url);
        let app_url = bucket_url + "/" + &package.name + ".json";

        let response = match ureq::get(&app_url).call() {
            Ok(out) => out,
            Err(err) => {
                log::error!("\n\nCould not get app '{}' from url:{app_url}, check the following:\n1. You are connected to the internet\n2. The package is correctly spelled / it exists\n3. If you are using a custom bucket check that there is a bucket dir and its on the master branch\n\n",package.name);
                log::debug!("Recieved error from github was:{err}");
                return Err(Box::new(ShellError::ResponseError(err.to_string())));
            }
        };

        let actual_manifest = match response.into_string() {
            Ok(out) => out,
            Err(err) => {
                log::error!("Got an invalid answer from github! {err}");
                return Err(Box::new(ShellError::ResponseError(err.to_string())));
            }
        };

        let parsed_manifest = match parsing::Manifest::from_str(
            &actual_manifest,
            package.name.to_string(),
            app_url.clone(),
            package.version.to_string(),
        ) {
            Ok(out) => out,
            Err(err) => {
                log::error!("Manifest body:{actual_manifest}\nGot an invalid manifest!");
                return Err(err);
            }
        };
        manifest_vec.push(parsed_manifest);
    }

    for man in &manifest_vec {
        install_cleanly(&man)?;
    }

    cleanup::cleanup_shims()?;
    venv::make_venv(manifest_vec)?;
    cleanup::cleanup_shims()?;

    Ok(())
}
