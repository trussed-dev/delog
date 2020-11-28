#[macro_use]
extern crate delog;

use delog::flushers::StdoutFlusher;

cfg_if::cfg_if! {
    if #[cfg(feature = "verbose-renderer")] {
        use delog::renderers::RipgrepRenderer;
        delog!(Delogger, 4096, StdoutFlusher, renderer: RipgrepRenderer);
        static RENDERER: RipgrepRenderer = RipgrepRenderer {};
    } else {
        use delog::renderers::ArgumentsRenderer;
        delog!(Delogger, 4096, StdoutFlusher);
        static RENDERER: ArgumentsRenderer = ArgumentsRenderer {};
    }
}

static FLUSHER: StdoutFlusher = StdoutFlusher {};

fn main() {
    Delogger::init(delog::LevelFilter::Info, &FLUSHER, &RENDERER).expect("all good");
    lib_a::f();
    lib_b::g();
    println!("{:?}", delog::trylogger().unwrap().statistics());
    Delogger::flush();
    println!("{:?}", delog::trylogger().unwrap().statistics());
}
