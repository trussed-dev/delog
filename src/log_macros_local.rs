// There is a syntax issue with "repetitions in binding patterns for nested macros",
// with a workaround: https://github.com/rust-lang/rust/issues/35853#issuecomment-443110660
//
// This is why we want to  have `($)` expressions in the following, can just imagine they're not there.
//
// Unfortunately, I couldn't get this to work, so instead we use the weird `with_dollar_sign!` instead.

#[macro_export]
#[doc(hidden)]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

/// Generate logging macros that can be gated by library.
///
/// Realize that these macros are generated **in the namespace of the consuming library**, the one
/// that actally later makes calls to `local_warn!` etc.
///
/// To see this in action, compile documentation using `cargo doc --features example`, or inspect
/// the `gate-tests/` subdirectory.
#[macro_export]
macro_rules! local_macros {
    () => {
        $crate::with_dollar_sign! {
            ($d:tt) => {

                /// Local version of `log!`.
                #[macro_export(local_inner_macros)]
                macro_rules! log {
                    (target: $target:expr, $lvl:expr, $message:expr) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                        $crate::upstream::log!(target: $target, $lvl, $message));
                    (target: $target:expr, $lvl:expr, $d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                        $crate::upstream::log!(target: $target, $lvl, $d($arg)+));
                    ($lvl:expr, $d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                        $crate::upstream::log!($lvl, $d($arg)+));
                }

                /// Local version of `debug!`.
                #[macro_export(local_inner_macros)]
                macro_rules! debug {
                    (target: $target:expr, $d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-debug"), not(feature = "log-none")))]
                        $crate::upstream::debug!(target: $target, $d($arg)+));
                    ($d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-debug"), not(feature = "log-none")))]
                        $crate::upstream::debug!($d($arg)+));
                }

                /// Local version of `error!`.
                #[macro_export(local_inner_macros)]
                macro_rules! error {
                    (target: $target:expr, $d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-error"), not(feature = "log-none")))]
                        $crate::upstream::error!(target: $target, $d($arg)+));
                    ($d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-error"), not(feature = "log-none")))]
                        $crate::upstream::error!($d($arg)+));
                }

                /// Local version of `info!`.
                #[macro_export(local_inner_macros)]
                macro_rules! info {
                    (target: $target:expr, $d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                        $crate::upstream::info!(target: $target, $d($arg)+));
                    ($d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                        $crate::upstream::info!($d($arg)+));
                }

                /// Local version of `trace!`.
                #[macro_export(local_inner_macros)]
                macro_rules! trace {
                    (target: $target:expr, $d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-trace"), not(feature = "log-none")))]
                        $crate::upstream::trace!(target: $target, $d($arg)+));
                    ($d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-trace"), not(feature = "log-none")))]
                        $crate::upstream::trace!($d($arg)+));
                }

                /// Local version of `warn!`.
                #[macro_export(local_inner_macros)]
                macro_rules! warn {
                    (target: $target:expr, $d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-warn"), not(feature = "log-none")))]
                        $crate::upstream::warn!(target: $target, $d($arg)+));
                    ($d($arg:tt)+) => (
                        #[cfg(all(any(feature = "log-all", feature = "log-warn"), not(feature = "log-none")))]
                        $crate::upstream::warn!($d($arg)+));
                }

                /// Immediate version of `log!`.
                macro_rules! log_now {
                    ($lvl:expr, $d($arg:tt)+) => (
                        log!(target: "!", $lvl, $d($arg)+)
                    );
                }

                /// Immediate version of `debug!`.
                macro_rules! debug_now {
                    ($d($arg:tt)+) => (
                        debug!(target: "!", $d($arg)+)
                    );
                }

                /// Immediate version of `error!`.
                macro_rules! error_now {
                    ($d($arg:tt)+) => (
                        error!(target: "!", $d($arg)+)
                    );
                }

                /// Immediate version of `info!`.
                macro_rules! info_now {
                    ($d($arg:tt)+) => (
                        info!(target: "!", $d($arg)+)
                    );
                }

                /// Immediate version of `trace!`.
                macro_rules! trace_now {
                    ($d($arg:tt)+) => (
                        trace!(target: "!", $d($arg)+)
                    );
                }

                /// Immediate version of `warn!`.
                macro_rules! warn_now {
                    ($d($arg:tt)+) => (
                        warn!(target: "!", $d($arg)+)
                    );
                }

                // /// Fallible version of `info!`.
                // #[macro_export(local_inner_macros)]
                // macro_rules! try_info {
                //     (target: $target:expr, $($arg:tt)+) => (
                //         try_log!(target: $target, $crate::Level::Info, $($arg)+)
                //     );
                //     ($($arg:tt)+) => (
                //         try_log!($crate::Level::Info, $($arg)+)
                //     )
                // }

                // /// Fallible version of `log!`.
                // #[macro_export(local_inner_macros)]
                // macro_rules! try_log {
                //     (target: $target:expr, $lvl:expr, $message:expr) => (
                //         #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                //         $crate::global_try_log!(target: $target, $lvl, $message));
                //     (target: $target:expr, $lvl:expr, $d($arg:tt)+) => (
                //         #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                //         $crate::upstream::log!(target: $target, $lvl, $d($arg)+));
                //     ($lvl:expr, $d($arg:tt)+) => (
                //         #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                //         $crate::upstream::log!($lvl, $d($arg)+));
                // }

                // /// Fallible version of `info!`.
                // #[macro_export(local_inner_macros)]
                // macro_rules! try_info {

                //     (target: $target:expr, $d($arg:tt)+) => (
                //         #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                //         {
                //             $crate::global_try_info!(target: $target, $d($arg)+)
                //         }
                //         #[cfg(not(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none"))))]
                //         {
                //             Ok(())
                //         }
                //     );

                //     ($d($arg:tt)+) => (
                //         #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                //         {
                //             $crate::global_try_info!($d($arg)+)
                //         }
                //         #[cfg(not(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none"))))]
                //         {
                //             Ok(())
                //         }
                //     );
                // }

                /// Fallible version of `debug!`.
                #[cfg(all(any(feature = "log-all", feature = "log-debug"), not(feature = "log-none")))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_debug {

                    (target: $target:expr, $d($arg:tt)+) => (
                            $crate::global_try_debug!(target: $target, $d($arg)+)
                    );

                    ($d($arg:tt)+) => (
                            $crate::global_try_debug!($d($arg)+)
                    );
                }

                /// Fallible version of `debug!`.
                #[cfg(not(all(any(feature = "log-all", feature = "log-debug"), not(feature = "log-none"))))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_debug {

                    // (target: $target:expr, $d($arg:tt)+) => ( Ok() );

                    ($d($arg:tt)+) => ( core::result::Result::<(), ()>::Ok(()) );
                }

                /// Fallible version of `error!`.
                #[cfg(all(any(feature = "log-all", feature = "log-error"), not(feature = "log-none")))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_error {

                    (target: $target:expr, $d($arg:tt)+) => (
                            $crate::global_try_error!(target: $target, $d($arg)+)
                    );

                    ($d($arg:tt)+) => (
                            $crate::global_try_error!($d($arg)+)
                    );
                }

                /// Fallible version of `error!`.
                #[cfg(not(all(any(feature = "log-all", feature = "log-error"), not(feature = "log-none"))))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_error {

                    // (target: $target:expr, $d($arg:tt)+) => ( Ok() );

                    ($d($arg:tt)+) => ( core::result::Result::<(), ()>::Ok(()) );
                }

                /// Fallible version of `info!`.
                #[cfg(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none")))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_info {

                    (target: $target:expr, $d($arg:tt)+) => (
                            $crate::global_try_info!(target: $target, $d($arg)+)
                    );

                    ($d($arg:tt)+) => (
                            $crate::global_try_info!($d($arg)+)
                    );
                }

                /// Fallible version of `info!`.
                #[cfg(not(all(any(feature = "log-all", feature = "log-info"), not(feature = "log-none"))))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_info {

                    // (target: $target:expr, $d($arg:tt)+) => ( Ok() );

                    ($d($arg:tt)+) => ( core::result::Result::<(), ()>::Ok(()) );
                }

                /// Fallible version of `trace!`.
                #[cfg(all(any(feature = "log-all", feature = "log-trace"), not(feature = "log-none")))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_trace {

                    (target: $target:expr, $d($arg:tt)+) => (
                            $crate::global_try_trace!(target: $target, $d($arg)+)
                    );

                    ($d($arg:tt)+) => (
                            $crate::global_try_trace!($d($arg)+)
                    );
                }

                /// Fallible version of `trace!`.
                #[cfg(not(all(any(feature = "log-all", feature = "log-trace"), not(feature = "log-none"))))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_trace {

                    // (target: $target:expr, $d($arg:tt)+) => ( Ok() );

                    ($d($arg:tt)+) => ( core::result::Result::<(), ()>::Ok(()) );
                }

                /// Fallible version of `warn!`.
                #[cfg(all(any(feature = "log-all", feature = "log-warn"), not(feature = "log-none")))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_warn {

                    (target: $target:expr, $d($arg:tt)+) => (
                            $crate::global_try_warn!(target: $target, $d($arg)+)
                    );

                    ($d($arg:tt)+) => (
                            $crate::global_try_warn!($d($arg)+)
                    );
                }

                /// Fallible version of `warn!`.
                #[cfg(not(all(any(feature = "log-all", feature = "log-warn"), not(feature = "log-none"))))]
                #[macro_export(local_inner_macros)]
                macro_rules! try_warn {

                    // (target: $target:expr, $d($arg:tt)+) => ( Ok() );

                    ($d($arg:tt)+) => ( core::result::Result::<(), ()>::Ok(()) );
                }

            }
        }
    }
}
