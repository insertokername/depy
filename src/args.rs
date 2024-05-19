use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// If set prints out debuging info
    #[arg(short, long)]
    pub verbose: bool,

    /// If set doesn't initialize depy before running other commands. Read more about this in README/
    #[arg(short, long)]
    pub no_init: bool,
}
