#[macro_use]
extern crate log;

#[macro_use]
mod macros;

mod error;
mod random_ramble;

pub use error::Error;
pub use random_ramble::RandomRamble;
