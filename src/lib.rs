#[macro_use]
extern crate error_chain;
#[cfg(feature = "cli")]
#[macro_use]
extern crate log;

#[cfg(feature = "cli")]
#[doc(hidden)]
pub(crate) mod cli;
#[doc(hidden)]
pub(crate) mod errors;
#[doc(inline)]  
pub mod matching;
#[doc(inline)]  
pub mod qgram;
#[doc(inline)]  
pub mod verification;
