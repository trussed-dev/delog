//! Typical flushers in various environments.
//!
//! An actual firmware will likely want to implement its own flusher.
//!
//! Availability based on cargo flags. The `flushers` feature must always be
//! selected. Additionally setting `std` gives stdout/stderr flushers,
//! while additionally setting `semihosting` gives flushers to host's stdout/stderr.

#[cfg(any(feature = "std", test))]
mod std;
#[cfg(any(feature = "std", test))]
pub use crate::flushers::std::*;

#[cfg(feature = "semihosting")]
mod semihosting;
#[cfg(feature = "semihosting")]
pub use crate::flushers::semihosting::*;
