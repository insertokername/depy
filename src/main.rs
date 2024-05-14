use std::process::exit;

use manifest::Manifest;

mod manifest;
mod path;
mod shell;

fn main() {
    let man = if let Ok(man) = Manifest::from_str(include_str!("example.json")){
        man
    }
    else{
        println!("Couldn't parse json manifest!");      
        exit(1);
    };


    println!("{:#?}", man.paths);
    // shell::make_shell();

    // let teste: serde_json::Value = serde_json::from_value(serde_json::from_str("jkf")).unwrap();
}
