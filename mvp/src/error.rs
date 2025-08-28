use thiserror::Error;

#[derive(Error, Debug)]
pub enum MvpError {
    #[error("Tera Error: {0}")]
    Tera(#[from] tera::Error),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Custom Error: {0}")]
    Custom(String),
}
