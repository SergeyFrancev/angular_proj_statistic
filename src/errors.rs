use {
    thiserror::Error,
    std::path::PathBuf,
};

#[derive(Debug, Error)]
pub enum NgStatError {
    #[error("File not found {0:?}")]
    FileNotFound(Vec<PathBuf>),

    // #[error("File not found {0:?}")]
    // Tar(Vec<PathBuf>),
    // #[error("No log file found")]
    // NoLogFileFound,
    // #[error("Path not found: {0:?}")]
    // PathNotFound(PathBuf),
    // #[error("IO error: {0:?}")]
    // Io(#[from] io::Error),
    // #[error("Date time parsing error: {0:?}")]
    // DateTime(#[from] ParseDateTimeError),
    // #[error("status filter parsing error: {0:?}")]
    // StatusFilter(#[from] ParseStatusFilterError),
    // #[error("String filter parsing error: {0:?}")]
    // StrFilter(#[from] ParseStrFilterError),
    // #[error("time filter parsing error: {0:?}")]
    // TimeFilter(#[from] ParseTimeFilterError),
}