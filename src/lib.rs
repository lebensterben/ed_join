#[macro_use]
extern crate error_chain;
#[cfg(feature = "cli")]
#[macro_use]
extern crate log;

#[cfg(feature = "cli")]
pub(crate) mod cli;
pub(crate) mod errors;
pub mod matching;
pub mod qgram;
pub mod verification;
