//! Typical flushers in various environments.
//!
//! An actual firmware will likely want to implement its own flusher.
//!
//! Availability based on cargo flags. The `flushers` feature must always be
//! selected. Additionally setting `std` gives stdout/stderr flushers,
//! while additionally setting `semihosting` gives flushers to host's stdout/stderr.

// #[cfg(any(feature = "std-example-flushers", test))]
// mod std;
// #[cfg(any(feature = "std-example-flushers", test))]
// pub use crate::flushers::std::*;

// // #[cfg(feature = "semihosting-example-flushers")]
// // mod semihosting;
// // #[cfg(feature = "semihosting-example-flushers")]
// // pub use crate::flushers::semihosting::*;

use crate::Flusher;

#[derive(Debug, Default)]
/// Flushes logs to stdout.
pub struct StdoutFlusher {}

impl Flusher for StdoutFlusher {
    fn flush(&self, logs: &str) {
        print!("{}", logs);
    }
}

#[derive(Debug, Default)]
/// Flushes logs to stderr.
pub struct StderrFlusher {}

impl Flusher for StderrFlusher {
    fn flush(&self, logs: &str) {
        eprint!("{}", logs);
    }
}

