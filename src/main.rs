use crate::{dir::cleanup_shims, shell::make_devshell};

mod dir;
mod env_var;
mod manifest;
mod parse_json_manifest;
mod path;
mod shell;

fn main() {
    // let man = if let Ok(man) = Manifest::from_str(include_str!("example.json")) {
    //     man
    // } else {
    //     println!("Couldn't parse json manifest!");
    //     exit(1);
    // };

    // println!("{:#?}", man.env_vars);
    // let _ = shell::clean_install("python", "3.12.3");
    // let _ = shell::clean_install("nodejs", "22.2.0");
    
    shell::init_depy().unwrap();
    let man = manifest::Manifest::from_str(
        include_str!("python.json"),
        "python".to_string(),
        "3.12.0".to_string(),
    )
    .expect("Got an invalid manifest!");
    let man1 = manifest::Manifest::from_str(
        include_str!("nodejs.json"),
        "nodejs".to_string(),
        "22.2.0".to_string(),
    )
    .expect("Got an invalid manifest!");

    let man2 = manifest::Manifest::from_str(
        include_str!("grep.json"),
        "grep".to_string(),
        "3.11".to_string(),
    )
    .expect("Got an invalid manifest!");

    let _ = shell::clean_install(&man.name, &man.version);
    let _ = shell::clean_install(&man1.name, &man1.version);
    let _ = shell::clean_install(&man2.name, &man2.version);
    cleanup_shims().unwrap();

    make_devshell(vec![man, man1, man2]).expect("couldn't create devshel!");
    cleanup_shims().unwrap();
}
