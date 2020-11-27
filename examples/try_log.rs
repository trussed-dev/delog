use delog::{try_info, try_warn};

#[cfg(not(all(feature = "flushers", feature = "std")))]
compile_error!("This example needs the `flushers` and `std` features");

use delog::flushers::StdoutFlusher;

delog::delog!(Delogger, 64, StdoutFlusher);

static STDOUT_FLUSHER: StdoutFlusher = StdoutFlusher {};

fn main() {
    Delogger::init(log::LevelFilter::Info, &STDOUT_FLUSHER).ok();

    // do some serious work
    try_warn!("This is a warning").unwrap();
    try_info!("This is information").unwrap();
    try_warn!("This is a warning").unwrap();
    try_info!("This is information").expect_err("should error out due to incapacity");

    // flush the logs
    Delogger::flush();
}

