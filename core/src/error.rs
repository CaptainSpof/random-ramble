use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Jabber {
    #[error("Oh shit, here we go again…")]
    Custom(String),
    #[error("We ain't found shit!")]
    IO(#[from] io::Error),
    #[error("Walkdir · We ain't found shit!")]
    Walkdir(#[from] walkdir::Error),
    #[error("We ain'")]
    Tera(#[from] tera::Error),
}
