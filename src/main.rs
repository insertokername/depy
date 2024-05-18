#![allow(dead_code)]

mod dir;
mod env_var;
mod json_installer;
mod manifest;
mod parse_json_manifest;
mod shell;

fn main() {
    let depy_contents = std::fs::read_to_string("./depy.json").expect("Something went wrong while reading depy file, make sure it exists and has proper read privileges!\n");
    let json_value =
        serde_json::from_str(&depy_contents).expect("depy json was improperly formated!");
    json_installer::install(json_value).unwrap();
}
