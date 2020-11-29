#[macro_use]
extern crate delog;

use delog::example::StdoutFlusher;

delog!(Delogger, 256, StdoutFlusher);
generate_macros!();

static FLUSHER: StdoutFlusher = StdoutFlusher {};

fn main() {
    Delogger::init_default(delog::LevelFilter::Info, &FLUSHER).ok();

    // do some serious work
    warn!("This is a warning");
    info!(target: "!", "This is an IMMEDIATE information");
    info_now!("This is another IMMEDIATE information");
    info!("jeez '{:02X}'", delog::hex_str!(&[0xa1u8, 0xfF, 0x03]));
    info!("heeb '{:#02X?}'", [0xa1u8, 0xfF, 0x03].as_ref());
    info!("heeg '{:02X?}'", [0xa1u8, 0xfF, 0x03].as_ref());

    // flush the logs
    Delogger::flush();
}

