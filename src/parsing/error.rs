#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Error: Couldn't create a manifest from a given json!\nGot error {0}")]
    ManifestCreateError(String),
    #[error("Error: Couldn't parse an environment variable from a given json!\nFormat error was {0}")]
    EnvVariableFormatError(String),

    #[error("Error: Couldn't parse an env_add_path from a given json!\nFormat error was {0}")]
    EnvPathFormatError(String),
    #[error("Error: Improperly formated bin attribute!\nFormat error was {0}")]
    BinFormatError(String),
    #[error("Error: Couldn't find a version attribute")]
    MissingVersionError,
    #[error("Error: Couldn't read manifest: {0}!\nGot error: {1}")]
    ManifestReadError(String, String),
    #[error("Error: Couldn't parse manifest: {0}!\nGot error: {1}")]
    ManifestParseError(String, String),
}
