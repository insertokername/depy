pub mod args;
pub mod dir;
pub mod env_var;
pub mod json_installer;
pub mod manifest;
pub mod package;
pub mod parse_json_manifest;
pub mod shell;

#[macro_use]
extern crate lazy_static;

use clap::Parser;


lazy_static! {
    pub static ref ARGS: args::Args = args::Args::parse();
}