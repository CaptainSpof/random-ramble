//! Rambling randomly about stuff.
//!
//! The `random-ramble` crate provides an API to generates random patterns
//! given a `Tera` template and some inputs to choose from.
//!

#[macro_use]
extern crate log;

#[macro_use]
mod macros;

mod error;
mod random_ramble;

pub use crate::error::RambleError;
pub use crate::random_ramble::{refactor, RandomRamble};
