#![allow(dead_code)]

mod dir;
mod env_var;
mod json_installer;
mod manifest;
mod parse_json_manifest;
mod shell;

fn main() {

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format_target(false)
        .format_timestamp(None)
        .init();

    let depy_contents = std::fs::read_to_string("./depy.json").expect("Something went wrong while reading depy file, make sure it exists and has proper read privileges!\n");
    let json_value =
        serde_json::from_str(&depy_contents).expect("depy json was improperly formated!");
        
    shell::init_depy().unwrap();
    json_installer::install(json_value).unwrap();
}
