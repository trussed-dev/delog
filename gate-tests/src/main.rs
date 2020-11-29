#[macro_use]
extern crate delog;

use delog::example::StdoutFlusher;

cfg_if::cfg_if! {
    if #[cfg(feature = "verbose-renderer")] {
        use delog::render::RipgrepRenderer;
        delog!(Delogger, 4096, StdoutFlusher, renderer: RipgrepRenderer);
        static RENDERER: RipgrepRenderer = RipgrepRenderer {};
    } else {
        use delog::render::DefaultRenderer;
        delog!(Delogger, 4096, StdoutFlusher);
        static RENDERER: DefaultRenderer = DefaultRenderer {};
    }
}

static FLUSHER: StdoutFlusher = StdoutFlusher {};

fn main() {
    Delogger::init(delog::LevelFilter::Info, &FLUSHER, &RENDERER).expect("all good");
    lib_a::f();
    lib_b::g();
    println!("{:?}", delog::logger().unwrap().statistics());
    Delogger::flush();
    println!("{:?}", delog::logger().unwrap().statistics());
}
