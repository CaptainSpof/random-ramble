use std::fmt;

#[derive(Debug)]
pub enum Error {
    Custom(String)
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Custom(format!("Fuck, io error: {}", e.to_string()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Custom(e) => write!(f, "oh shit: {}", e)
        }
    }
}

impl std::error::Error for Error {}
