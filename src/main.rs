use crate::{dir::cleanup_shims, shell::init_depy};

mod tests;
mod env_var;
mod manifest;
mod parse_json_manifest;
mod path;
mod shell;
mod dir;

fn main() {
    // let man = if let Ok(man) = Manifest::from_str(include_str!("example.json")) {
    //     man
    // } else {
    //     println!("Couldn't parse json manifest!");
    //     exit(1);
    // };

    // println!("{:#?}", man.env_vars);
    init_depy().unwrap();
    let temp = shell::clean_install("python", "3.12.3");
    let temp = shell::clean_install("nodejs", "22.2.0");
    print!("{:#?}",temp);
    cleanup_shims().unwrap();
    
    // let teste: serde_json::Value = serde_json::from_value(serde_json::from_str("jkf")).unwrap();
}
