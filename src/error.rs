use thiserror::Error;

/// Result type for flour
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for flour
#[derive(Error, Debug)]
pub enum Error {
    /// Error passed from std::io
    #[error("System error: {0}")]
    IOError(std::io::Error),

    /// Error passed from semver crate
    #[error("Error parsing version: {0}")]
    SemverError(semver::Error),

    #[error("Error when parsing string: {0}")]
    StringUtf8Error(std::string::FromUtf8Error),

    /// Error called when option `--labels` is given to a non-BRCAD file
    #[error("--labels can only be used for BRCAD files")]
    LabelsOnNotBRCAD,

    #[error("Tried to use {0}, which is not implemented yet")]
    NonImplementedFeature(String),

    /// BRCAD labels file is invalid
    #[error("Failed to parse labels file")]
    BadLabelsFile,

    /// BRCAD labels file is not Shift-JIS
    #[error("Could not decode labels file from Shift-JIS!")]
    LabelsFileNotShiftJIS,

    /// Interpreting a BXCADWrapper from an incompatible version
    #[error(
        "This file was made with an incompatible flour version: {0}\n\
        flour can read files made from version {} up to the current version",
        crate::bxcad::OLDEST_SUPPORTED_VERSION
    )]
    IncompatibleVersion(semver::Version),

    /// File is not a BXCAD
    #[error("File given is not a known BXCAD file")]
    NotBXCAD,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::StringUtf8Error(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::IOError(std::io::Error::from(err))
    }
}

impl From<semver::Error> for Error {
    fn from(err: semver::Error) -> Self {
        Self::SemverError(err)
    }
}
