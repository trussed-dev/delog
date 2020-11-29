delog::generate_macros!();

pub fn f() {
    submodule::sub_f();
    lib_a1::f();
}

pub mod submodule {
    pub fn sub_f() {
        log::info!("global info log from lib_a");
        log::warn!("global info log from lib_a");

        // log!(delog::Level::Info, "log level info from lib_a::f");
        // log!(target: "!", delog::Level::Info, "log level info from lib_a::f");
        // log!(target: "!", delog::Level::Warn, "log level warn from lib_a::f");

        info!("local info from lib_a::f");
        warn!("local warn from lib_a::f");
        info_now!("immediate local info from lib_a::f");
        warn_now!("immediate local warn from lib_a::f");
    }
}
