use depy::{bucket, installer, parse_json, shell, ARGS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_level(if ARGS.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .format_target(false)
        .format_timestamp(None)
        .init();

    if ARGS.only_initialize {
        shell::init_depy()?;
        return Ok(());
    }

    if ARGS.delete {
        shell::uninstall_depy()?;
        return Ok(());
    }

    if ARGS.garbage_clean {
        shell::clean_depy_packages()?;
        return Ok(());
    }

    if ARGS.path_clean {
        shell::cleanup_path()?;
        return Ok(());
    }

    if ARGS.search.is_some() || ARGS.deep_search.is_some() {
        let query = ARGS
            .search
            .clone()
            .unwrap_or_else(|| ARGS.deep_search.clone().unwrap());
        println!("query: '{query}'");
        println!(
            "Found following packages: {:#?}",
            bucket::query_local_buckets(&query, ARGS.deep_search.is_some()).unwrap()
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
