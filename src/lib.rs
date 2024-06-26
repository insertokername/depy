pub mod args;
pub mod bucket;
pub mod dir;
pub mod env_var;
pub mod installer;
pub mod manifest;
pub mod package;
pub mod parse_json;
pub mod shell;

#[macro_use]
extern crate lazy_static;

use clap::Parser;


lazy_static! {
    pub static ref ARGS: args::Args = args::Args::parse();
}
