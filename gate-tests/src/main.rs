#[macro_use]
extern crate delog;

use delog::flushers::StdoutFlusher;

delog!(Delogger, 4096, StdoutFlusher);

static FLUSHER: StdoutFlusher = StdoutFlusher {};

fn main() {
    Delogger::init(delog::LevelFilter::Info, &FLUSHER).expect("all good");
    lib_a::f();
    lib_b::g();
    println!("{:?}", delog::trylogger().unwrap().statistics());
    Delogger::flush();
    println!("{:?}", delog::trylogger().unwrap().statistics());
}
