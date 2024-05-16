use crate::dir;

use std::io::{self, BufRead,Write};

/// enters a dev shell with all environment variables set
pub fn enter_shell() -> Result<String, Box<dyn std::error::Error>> {
    dir::init_depy_dir()?; // makes the %userprofile%/depy/scoop dirs if not allready existing

    let handle = duct::cmd!("cmd", "/C", "scoop bucket add main & scoop uninstall python")
        .env("SCOOP", dir::get_depy_dir_location())
        .env(
            "shit",
            std::env::var("PATH").expect("PATH variable not set, this should always be set"),
        )
        .stderr_to_stdout()
        .reader()
        .unwrap();
        // .read()
        // .unwrap())

    // provide real time rendering of the stdout of the command
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();

    let reader = std::io::BufReader::new(handle);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                writeln!(stdout_lock, "{}", line).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    };
    Ok("DONE".to_string())
    // Ok(duct::cmd!("cmd", "/C", "echo caca & echo oterh caca").read().unwrap())
}
