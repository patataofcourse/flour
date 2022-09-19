use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("System error: {0}")]
    IOError(std::io::Error),
    #[error("Error parsing version: {0}")]
    SemverError(semver::Error),
    #[error("Error when parsing string: {0}")]
    StringUtf8Error(std::string::FromUtf8Error),
    #[error("--labels can only be used for BRCAD files")]
    LabelsOnNotBRCAD,
    #[error("Tried to use {0}, which is not implemented yet")]
    NonImplementedFeature(&'static str),
    #[error("Failed to parse labels file")]
    BadLabelsFile,
    #[error("Could not decode labels file from Shift-JIS!")]
    LabelsFileNotShiftJIS,
    #[error("Incompatible version {0}")]
    IncompatibleVersion(semver::Version),
    #[error("File given is not a BXCAD file")]
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
