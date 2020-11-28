#[macro_use]
extern crate delog;

use delog::upstream::info;

use delog::flushers::StdoutFlusher;

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
