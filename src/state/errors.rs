use std::path::PathBuf;

use thiserror::Error;

use super::environment::EnvVariable;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("Failed to get external environment variable: {0}")]
    MissingExternalEnvironmentVariable(EnvVariable),
    #[error("Failed to get internal environment variable: {0}")]
    MissingInternalEnvironmentVariable(EnvVariable),
    #[error("Failed to update shell environment variable: {0}")]
    FailedToUpdateEnvironmentVariable(EnvVariable),
    #[error("Previous directory does not exist")]
    NoPreviousDirectory,
    #[error("Next directory does not exist")]
    NoNextDirectory,
    #[error("Failed to open configuration file: {0}")]
    FailedToOpenConfigFile(PathBuf),
    #[error("Failed to read configuration file: {0}")]
    FailedToReadConfigFile(PathBuf),
    #[error("Unknown error")]
    Uncategorized,
}

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Failed to convert PathBuf to String: {0}")]
    FailedToConvertPathBufToString(PathBuf),
    #[error("Failed to canonicalize directory path: {0}")]
    FailedToCanonicalize(PathBuf),
    #[error("Failed to access directory path: {0}")]
    FailedToAccess(PathBuf),
    #[error("Directory does not exist: {0}")]
    UnknownDirectory(PathBuf),
}
