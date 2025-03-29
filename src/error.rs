use thiserror::Error;

#[derive(Error, Debug)]
pub enum LangCheckerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Path not found: {0}")]
    PathNotFound(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, LangCheckerError>;

