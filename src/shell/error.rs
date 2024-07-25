#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ShellError {
    #[error("Error: Couldn't execute a command!\nGot error: {0}")]
    ExecutionError(String),

    #[error("Error: Couldn't initialize depy!\n Got error {0}\n\nPlease make sure scoop is installed on your system!")]
    InitializeError(String),
    #[error("Error: Couldn't update the scoop instalation inside depy!\nGot scoop output {0}\nMake sure scoop is installed and you are connected to the internet!")]
    UpdateError(String),
    #[error("Error: Couldn't install an application!\n{0}\n\nFailed to install {1}, scoop error above ^^^^^^^^^^^^^^^^\n\nMake sure all required buckets are installed!")]
    SingleInstallError(String, String),

    #[error("Error: Couldn't delete a file or folder!\nGot error{0}")]
    DeleteError(String),
    #[error("Error: Couldn't make shims for{0}!\nGot error:{1}")]
    MakeShimError(String, String),
    #[error("Error: Couldn't create .depyenv folder!\nGot error{0}")]
    CreateEnvError(String),
    #[error("Error: Couldn't copy shims!\nGot error:{0}")]
    CopyShimError(String),
    #[error("Error: Couldn't write to a file!\nGot error{0}")]
    WriteError(String),

    #[error("Error: Couldn't read from {0}!\nGot error{1}")]
    ReadError(String, String),
    #[error("Error: Couldn't uninstall {0}, error:{1}\nIf you want to try force uninstall run cli with the -g/-d and the -f flag")]
    PackageUninstallError(String, String),
    #[error("Error: Couldn't cleanup a packages!\nScoop errored out on:\n{0}\n\nFailed to cleanup paths, output above ^^^^^^^^^^^^^^^^")]
    CleanupError(String),

    #[error("Error: Couldn't clean buckets!\nGot error{0}")]
    CleanBucketError(String),
    #[error("Error: Couldn't add bucket name: {0} bucket url: {1}.\nGot error{2}")]
    AddBucketError(String, String, String),
    #[error("Error: Couldn't remove bucket name: {0}.\nGot error{1}!")]
    RemoveBucketError(String, String),
    #[error("Error: Couldn't determine the url of bucket {0}!\nGot error: {1}")]
    BucketUrlError(String, String),
    #[error("Error: Couldn't parse manifest: {0}!\nGot error: {1}")]
    ManifestParseError(String, String),

    #[error("Error: Invalid response recieved!\nGot error from github:{0}")]
    ResponseError(String),
}
