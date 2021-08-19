//! Error types

use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RambleError {
    #[error("Ah shit, here we go again…")]
    Custom(String),
    #[error("std::io says: We ain't found shit!")]
    IO(#[from] io::Error),
    #[error("walkdir says: We ain't found shit!")]
    Walkdir(#[from] walkdir::Error),
    #[error("Can't terraform a blackhole.")]
    Tera(#[from] tera::Error),
}

/// Convenient wrapper around std::Result.
pub type Result<T> = ::std::result::Result<T, RambleError>;
