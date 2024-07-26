//! Depy is a program that interacts with the [`scoop`](https://scoop.sh/) package manager
//! that atttempts to make clean installations of programs without modifying the use PATH
//! or leaving residual files on your system
//! 
//! # Examples:
//! 
//! installing from a depy.json file:
//! 
//! ```
//! use depy::{package, parsing, shell};
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     shell::install::init_depy()?;
//! 
//!     // all this does is that is parses a json to a serde_json::Value object
//!     let json_value = parsing::parse_json::read_json_file("./depy.json")?;
//! 
//!     // returns scoop packages to be installed from `./depy.json`
//!     let packages = package::Package::multiple_packages_from_json(&json_value)?;
//! 
//!     // installs all the packages using scoop
//!     shell::install::install(packages)?;
//!     Ok(())
//! }
//! ```

/// Represents scoop packages as structs
pub mod package;
/// Reading / Modifing json files
pub mod parsing;
/// Shell commands
pub mod shell;
