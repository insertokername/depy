#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PackageError {
    #[error("Error: Invalid package structure!\nExpected each package to be an package_object")]
    PackageStructureError,
    #[error("Error: Invalid bucket_url format!")]
    BucketUrlFormatError,
    #[error("Error: Invalid bucket_name format!")]
    BucketNameFormatError,
    #[error("Error: Invalid name format!")]
    NameFormatError,
    #[error("Error: Invalid version format!")]
    VersionFormatError,
    #[error("Error: Invalid depy.json format!")]
    PacakgeFormatError,
    #[error("Error: Couldn't save packages to ./depy.json!\nGot error{0}")]
    PacakgeSaveError(String),
    
}