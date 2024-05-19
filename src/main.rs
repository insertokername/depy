// #![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

use clap::Parser;

mod args;
mod dir;
mod env_var;
mod json_installer;
mod manifest;
mod package;
mod parse_json_manifest;
mod shell;

lazy_static! {
    pub static ref ARGS: args::Args = args::Args::parse();
}

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

    let depy_contents = match std::fs::read_to_string("./depy.json") {
        Ok(out) => out,
        Err(err) => {
            log::error!("Something went wrong while reading depy file, make sure it exists and has proper read privileges!\n");
            return Err(Box::new(err));
        }
    };

    let json_value = match serde_json::from_str(&depy_contents) {
        Ok(out) => out,
        Err(err) => {
            log::error!("depy json was improperly formated!");
            return Err(Box::new(err));
        }
    };

    if let Err(err) = json_installer::install(json_value) {
        log::error!("Error occured while installing from depy file!");
        return Err(err);
    }
    Ok(())
}
