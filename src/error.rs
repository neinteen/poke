use std::io;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Couldn't find file at: {0}")]
    FileNotFound(String),
    #[error("Filename contains illegal character: {0}")]
    IllegalCharacter(char),
    #[error("Windows doesn't allow the Filename: {0}")]
    IllegalName(String),
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
}
