#[macro_use]
extern crate delog;

use delog::log::info;

use delog::example::StdoutFlusher;

delog!(Delogger, 25, StdoutFlusher);

static STDOUT_FLUSHER: StdoutFlusher = StdoutFlusher {};

fn main() {
    Delogger::init_default(delog::LevelFilter::Info, &STDOUT_FLUSHER).ok();

    let msg = "1234567890";

    // this logs more than fits the buffer
    (0..10).for_each(|_| {
        info!("{}", msg);
        Delogger::flush();
    });
}
