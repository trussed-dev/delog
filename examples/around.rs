#[macro_use]
extern crate delog;

#[cfg(not(all(feature = "flushers", feature = "std")))]
compile_error!("This example needs the `flushers` and `std` features");

use delog::flushers::StdoutFlusher;

delog!(Delogger, 25, StdoutFlusher);

static STDOUT_FLUSHER: StdoutFlusher = StdoutFlusher {};

fn main() {
    Delogger::init(log::LevelFilter::Info, &STDOUT_FLUSHER).ok();

    let msg = "1234567890";

    (0..10).for_each(|_| {
        info!("{}", msg);
        Delogger::flush();
    });
}
