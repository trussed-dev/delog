delog::generate_macros!();

use delog::log;

pub fn g() {
    log::info!("global info from B");
    warn!("info from B");

    log!(delog::Level::Info, "log level info from B");
    log!(target: "!", delog::Level::Info, "log! ! info from B");
    log!(target: "!", delog::Level::Warn, "log! ! warn from B");

    info!("info from B");
    warn!("warn from B");
    error!("error from B");
    error!("another error from B");
    info!(target: "!", "immediate local info from B");
    warn!(target: "!", "immediate local warn from B");
}
