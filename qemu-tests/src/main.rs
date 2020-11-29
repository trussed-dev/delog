#![no_std]
#![no_main]

use cortex_m_semihosting::debug;
extern crate panic_semihosting;
use cortex_m_rt::entry;

#[macro_use]
extern crate delog;

#[derive(Debug, Default)]
pub struct SemihostingFlusher {}

impl delog::Flusher for SemihostingFlusher {
    fn flush(&self, logs: &str) {
        cortex_m_semihosting::hprint!("{}", logs).ok();
    }
}

delog::generate_macros!();

delog!(Delogger, 256, SemihostingFlusher);

static SEMIHOSTING_FLUSHER: SemihostingFlusher = SemihostingFlusher {};

fn test_runs() {
    // do some serious™ work
    warn!("This is a warning");
    info!(target: "!", "This is IMMEDIATE information");
    info_now!("This too");
    info!("hex_str '{}'", delog::hex_str!(&[0xa1u8, 0xfF, 0x03]));
    info!("alternate debug '{:#02X?}'", [0xa1u8, 0xff, 0x03].as_ref());
    info!("regular debug '{:02X?}'", [0xA1u8, 0xFF, 0x03].as_ref());

    // flush the logs
    Delogger::flush();
}

#[entry]
fn main() -> ! {

    Delogger::init_default(delog::LevelFilter::Debug, &SEMIHOSTING_FLUSHER).ok();

    test_runs();

    debug_now!("{:?}", delog::logger().unwrap().statistics());
    info_now!("All tests passed");

    debug::exit(debug::EXIT_SUCCESS);

    loop { continue; }

}
