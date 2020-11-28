delog::local_macros!();

pub fn f() {
    info!("global info log from lib_a");
    warn!("global info log from lib_a");

    log!(delog::Level::Info, "log level info from lib_a::f");
    log!(target: "!", delog::Level::Info, "log level info from lib_a::f");
    log!(target: "!", delog::Level::Warn, "log level warn from lib_a::f");

    info!("local info from lib_a::f");
    warn!("local warn from lib_a::f");
    info!(target: "!", "immediate local info from lib_a::f");
    warn!(target: "!", "immediate local warn from lib_a::f");
}
