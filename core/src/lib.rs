#[macro_use]
extern crate log;

#[macro_use]
mod macros;

mod error;
mod random_ramble;

pub use crate::error::Jabber;
pub use crate::random_ramble::{refactor, RandomRamble};
