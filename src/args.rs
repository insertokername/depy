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

    /// If set cleans up the folder %userprofile%/depy
    #[arg(short, long)]
    pub dir_cleanup: bool,

    // /// If set cleans up the path, should be used after program was forcefully closed
    // #[arg(short, long)]
    // pub path_cleanup: bool,

    /// If set delete packages that can't be uninstalled with del -f {package_name}
    #[arg(short, long)]
    pub force_uninstall: bool,
}