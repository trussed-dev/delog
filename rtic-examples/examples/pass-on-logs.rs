#![no_std]
#![no_main]

use panic_semihosting as _;

#[macro_use]
extern crate delog;
generate_macros!();

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, QEI0])]
mod app {

    delog!(Delogger, 2048, crate::lib::Flusher);

    #[resources]
    struct Resources {
        #[init(0)]
        x: u32,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        info!("in init (will be missed)");
        static FLUSHER: crate::lib::Flusher = crate::lib::Flusher {};
        Delogger::init_default(delog::LevelFilter::Debug, &FLUSHER).ok();
        info!("in init (will be logged, eventually)");
        init::LateResources {}
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        // hprintln!("in idle").ok();
        info!("in idle");
        Delogger::flush();

        info!("press Ctrl-A + X to abort QEMU");

        // periodic::schedule(c.start + 4_000_000.cycles()).unwrap();

        let mut i = 0;
        loop {
            for _ in 0..1_000_000 {
                cortex_m::asm::nop();
            }
            info!("logging at {}", i);
            i += 1;
            Delogger::flush();
        }
    }

    // unfortunately, doesn't work on QEMU
    //
    // #[task]
    // fn periodic(c: periodic::Context) {
    //     info!("logging at {}", c.start);
    //     periodic::schedule(c.start + 4_000_000.cycles()).unwrap();
    // }

    #[task]
    /// In this example, `Flusher` might as well output the logs directly;
    /// the point of the example is to show how to get the logs back inside RTIC
    /// to use its scheduling, for instance to drive a USB serial output.
    fn log(_: log::Context, logs: crate::types::Logs) {
        if logs.len() > 0 {
            cortex_m_semihosting::hprint!("===\n{}", &logs).ok();
        }
    }
}

mod lib {
    #[derive(Debug)]
    pub struct Flusher {}

    impl delog::Flusher for Flusher {
        fn flush(&self, logs: &str) {
            let logs = crate::types::Logs::from(logs);
            crate::app::log::spawn(logs).ok();
        }
    }
}

mod types {
    pub type Logs = heapless::String<heapless::consts::U2048>;
}

