#[macro_use]
extern crate delog;

use delog::flushers::StdoutFlusher;
use delog::renderers::RipgrepRenderer;
delog!(Delogger, 196, StdoutFlusher, renderer: RipgrepRenderer);
local_macros!();

static STDOUT_FLUSHER: StdoutFlusher = StdoutFlusher {};
static RENDERER: RipgrepRenderer = RipgrepRenderer {};

fn main() {
    Delogger::init(delog::LevelFilter::Info, &STDOUT_FLUSHER, &RENDERER).ok();

    // do some serious work
    global_try_warn!("This is a warning").unwrap();
    global_try_info!("This is information").unwrap();
    global_try_warn!("This is a warning").unwrap();
    global_try_info!("This is information").expect_err("should error out due to incapacity");

    // flush the logs
    Delogger::flush();

    println!("===");

    try_warn!("This is a warning").unwrap();
    try_info!("This is information").unwrap();
    try_warn!(target: "!", "This is a warning").unwrap();
    try_warn!("This is a warning").unwrap();
    #[cfg(not(feature = "log-none"))]
    try_info!("This is information").expect_err("should error out due to incapacity");
    #[cfg(feature = "log-none")]
    try_info!("This is information").ok();

    // flush the logs
    Delogger::flush();
}
