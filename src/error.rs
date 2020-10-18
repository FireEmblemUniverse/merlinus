use std::io;
use std::result;

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("type error")]
    TypeError,
    #[error("decode error")]
    DecodeError,
    #[error("error loading merlinus.toml: {0}")]
    ProjectParseError(#[from] toml::de::Error),
}
