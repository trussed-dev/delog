//! An example deferred logger, generated as
//! `delog!(Delogger, 4096, StdoutFlusher, ArgumentsRenderer)`.
//!
//! It is included here for documentation purposes only.
//!
//! Do ensure that the `example` feature is not active in production!
//!
//! ```
//! use delog::flushers::StdoutFlusher;
//! delog!(Delogger, 256, StdoutFlusher);
//! static STDOUT_FLUSHER: StdoutFlusher = StdoutFlusher {};
//! Delogger::init(log::LevelFilter::Info, &STDOUT_FLUSHER).ok();
//!
//! warn!("This is a warning");
//! info_now!("This is IMMEDIATE information");
//! info!("twenty-four bits '{}'", delog::hex_str!(&[0xa1u8, 0xfF, 0x03]));
//!
//! Delogger::flush();
//! ```

use crate::flushers::StdoutFlusher;
use crate::renderers::ArgumentsRenderer;

crate::delog!(Delogger, 4096, StdoutFlusher, renderer: ArgumentsRenderer);

crate::local_macros!();

