use depy::{bucket, installer, parse_json, shell};

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = depy::args::ArgsCli::parse();

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
        shell::init_depy()?;
        return Ok(());
    }

    if args.delete {
        shell::uninstall_depy(args.force_uninstall)?;
        return Ok(());
    }

    if args.garbage_clean {
        shell::clean_depy_packages(args.force_uninstall)?;
        return Ok(());
    }

    if args.path_clean {
        shell::cleanup_path()?;
        return Ok(());
    }

    if args.search.is_some() || args.deep_search.is_some() {
        let query = args
            .search
            .clone()
            .unwrap_or_else(|| args.deep_search.clone().unwrap());
        println!("query: '{query}'");
        println!(
            "Found following packages: {:#?}",
            bucket::query_local_buckets(&query, args.deep_search.is_some()).unwrap()
        );
        return Ok(());
    }

    shell::init_depy()?;

    let json_value = parse_json::read_json_file("./depy.json")?;

    let packages = depy::package::Package::multiple_packages_from_json(&json_value)?;

    if let Err(err) = installer::install(packages) {
        log::error!("Error occured while installing from depy file!");
        return Err(err);
    }
    Ok(())
}
