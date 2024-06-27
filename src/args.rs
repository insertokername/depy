use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// If set prints out debuging info
    #[arg(short, long)]
    pub verbose: bool,

    /// If set delete depy and uninstalls all programs installed with depy
    #[arg(short, long)]
    pub delete: bool,

    /// If set uinstalls all apps installed by depy
    #[arg(short, long)]
    pub garbage_clean: bool,


    // /// If set cleans up the path, should be used after program was forcefully closed
    // #[arg(short, long)]
    // pub path_cleanup: bool,

    /// If set initializes depy under %userprofile%/depy and exists
    #[arg(short='i', long)]
    pub only_initialize: bool,

    /// If set delete packages that can't be uninstalled with del -f {package_name}
    #[arg(short, long)]
    pub force_uninstall: bool,

    /// Take a package name to search for in the locally installed buckets
    #[arg(short, long)]
    pub search: Option<String>,

    /// Take a package name to search for in the locally installed buckets
    #[arg(short='S', long)]
    pub deep_search: Option<String>

}
