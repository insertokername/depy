//! This is the depy cli entry point of the program
//! 
//! This just calls basic depy library functions to take action on the given arguments
//! and also reads the ./depy.json file in the same folder and installs the programs in it
use clap::Parser;
use depy::{package, parsing, shell};

/// Arguments parsing
mod args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::ArgsCli::parse();

    env_logger::Builder::new()
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .format_target(false)
        .format_timestamp(None)
        .init();

    if args.only_initialize {
        shell::install::init_depy()?;
        return Ok(());
    }

    if args.delete {
        shell::cleanup::uninstall_depy(args.force_uninstall)?;
        return Ok(());
    }

    if args.garbage_clean {
        shell::cleanup::clean_depy_packages(args.force_uninstall)?;
        return Ok(());
    }

    if args.path_clean {
        shell::cleanup::cleanup_path()?;
        return Ok(());
    }

    if args.search.is_some() || args.deep_search.is_some() {
        let query = args
            .search
            .clone()
            .unwrap_or_else(|| args.deep_search.clone().unwrap());
        let pkgs = shell::bucket::query_all_buckets(&query, args.deep_search.is_some()).map_err(
            |err| {
                log::error!(
                    "Got an error while searchiing for packages: {}\n\n",
                    err.to_string()
                );
                err
            },
        )?;
        println!("query: '{query}'");
        println!("Found following packages:\n",);
        for pkg in pkgs {
            println!(
                "Name: {}\nFrom bucket: {}\nBucket url: {}\n\n",
                pkg.name, pkg.bucket_name, pkg.bucket_url
            )
        }
        return Ok(());
    }

    shell::install::init_depy()?;

    let json_value = parsing::parse_json::read_json_file("./depy.json")?;

    let packages = package::multiple_packages_from_json(&json_value)?;

    if let Err(err) = shell::install::install(packages) {
        log::error!("Error occured while installing from depy file!");
        return Err(err);
    }
    Ok(())
}
