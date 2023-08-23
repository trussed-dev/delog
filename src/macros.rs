/// Fallible (ungated) version of `log!`.
#[macro_export]
#[doc(hidden)]
#[cfg(not(feature = "std-log"))]
macro_rules! try_log {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= $crate::log::STATIC_MAX_LEVEL && lvl <= $crate::log::max_level() {
            $crate::__private_api_try_log(
                ::core::format_args!($($arg)+),
                lvl,
                &($target, ::core::module_path!(), ::core::file!(), ::core::line!()),
            )
        } else {
            Ok(())
        }
    });

    ($lvl:expr, $($arg:tt)+) => ($crate::try_log!(target: ::core::module_path!(), $lvl, $($arg)+))
}

#[macro_export]
#[doc(hidden)]
#[cfg(feature = "std-log")]
macro_rules! try_log {

    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        $crate::log::log!(target: $target, $lvl, $($arg)+);
        ::core::result::Result::<(), ()>::Ok(())
    });

    ($lvl:expr, $($arg:tt)+) => ({
        $crate::log::log!($lvl, $($arg)+);
        ::core::result::Result::<(), ()>::Ok(())
    });
}

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
macro_rules! generate_macros {
    () => {
        $crate::with_dollar_sign! {
            ($d:tt) => {

                // /// Fallible version of `debug!`.
                // #[cfg(all(any(feature = "log-all", feature = "log-debug"), not(feature = "log-none")))]
                #[macro_use] #[macro_export]
                #[doc(hidden)]
                macro_rules! try_debug {

                    (target: $target:expr, $d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-debug")) && !cfg!(feature = "log-none") {
                            $crate::try_log!(target: $target, $crate::Level::Debug, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );

                    ($d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-debug")) && !cfg!(feature = "log-none") {
                            $crate::try_log!($crate::Level::Debug, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );
                }

                /// Fallible version of `error!`.
                #[macro_use] #[macro_export]
                #[doc(hidden)]
                macro_rules! try_error {
                    (target: $target:expr, $d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-error")) && !cfg!(feature = "log-none") {
                            $crate::try_log!(target: $target, $crate::Level::Error, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );

                    ($d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-error")) && !cfg!(feature = "log-none") {
                            $crate::try_log!($crate::Level::Error, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );
                }

                /// Fallible version of `info!`.
                #[macro_use] #[macro_export]
                #[doc(hidden)]
                macro_rules! try_info {
                    (target: $target:expr, $d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-info")) && !cfg!(feature = "log-none") {
                            $crate::try_log!(target: $target, $crate::Level::Info, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );

                    ($d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-info")) && !cfg!(feature = "log-none") {
                            $crate::try_log!($crate::Level::Info, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );
                }

                /// Fallible version of `trace!`.
                #[macro_use] #[macro_export]
                #[doc(hidden)]
                macro_rules! try_trace {
                    (target: $target:expr, $d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-trace")) && !cfg!(feature = "log-none") {
                            $crate::try_log!(target: $target, $crate::Level::Trace, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );

                    ($d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-trace")) && !cfg!(feature = "log-none") {
                            $crate::try_log!($crate::Level::Trace, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );
                }

                /// Fallible version of `warn!`.
                #[macro_use] #[macro_export]
                #[doc(hidden)]
                macro_rules! try_warn {
                    (target: $target:expr, $d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-warn")) && !cfg!(feature = "log-none") {
                            $crate::try_log!(target: $target, $crate::Level::Warn, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );

                    ($d($arg:tt)+) => (
                        if (cfg!(feature = "log-all") || cfg!(feature = "log-warn")) && !cfg!(feature = "log-none") {
                            $crate::try_log!($crate::Level::Warn, $d($arg)+)
                        } else {
                            core::result::Result::<(), ()>::Ok(())
                        }
                    );
                }

                /// Local version of `log!`.
                #[macro_use] #[macro_export]
                #[doc(hidden)]
                macro_rules! log {
                    (target: $target:expr, $d($arg:tt)+) => {{
                        if !cfg!(feature = "log-none") {
                            $crate::try_log!(target: $target, $d($arg)+).ok();
                        }
                    }};
                    ($d($arg:tt)+) => {{
                        if !cfg!(feature = "log-none") {
                            $crate::try_log!($d($arg)+).ok();
                        }
                    }};
                }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Local version of `debug!`.
                macro_rules! debug { ($d($arg:tt)+) => ( try_debug!($d($arg)+).ok() ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Local version of `error!`.
                macro_rules! error { ($d($arg:tt)+) => ( try_error!($d($arg)+).ok() ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Local version of `info!`.
                macro_rules! info { ($d($arg:tt)+) => ( try_info!($d($arg)+).ok() ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Local version of `trace!`.
                macro_rules! trace { ($d($arg:tt)+) => ( try_trace!($d($arg)+).ok() ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Local version of `warn!`.
                macro_rules! warn { ($d($arg:tt)+) => ( try_warn!($d($arg)+).ok() ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Immediate version of `log!`.
                macro_rules! log_now {
                    ($lvl:expr, $d($arg:tt)+) => ( log!(target: "!", $lvl, $d($arg)+) );
                }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Immediate version of `debug!`.
                macro_rules! debug_now { ($d($arg:tt)+) => ( debug!(target: "!", $d($arg)+) ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Immediate version of `error!`.
                macro_rules! error_now { ($d($arg:tt)+) => ( error!(target: "!", $d($arg)+) ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Immediate version of `info!`.
                macro_rules! info_now { ($d($arg:tt)+) => ( info!(target: "!", $d($arg)+) ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Immediate version of `trace!`.
                macro_rules! trace_now { ($d($arg:tt)+) => ( trace!(target: "!", $d($arg)+) ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Immediate version of `warn!`.
                macro_rules! warn_now { ($d($arg:tt)+) => ( warn!(target: "!", $d($arg)+) ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Fallible immediate version of `log!`.
                macro_rules! try_log_now {
                    ($lvl:expr, $d($arg:tt)+) => ( try_log!(target: "!", $lvl, $d($arg)+) );
                }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Fallible immediate version of `debug!`.
                macro_rules! try_debug_now { ($d($arg:tt)+) => ( try_debug!(target: "!", $d($arg)+) ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Fallible immediate version of `error!`.
                macro_rules! try_error_now { ($d($arg:tt)+) => ( try_error!(target: "!", $d($arg)+) ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Fallible immediate version of `info!`.
                macro_rules! try_info_now { ($d($arg:tt)+) => ( try_info!(target: "!", $d($arg)+) ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Fallible immediate version of `trace!`.
                macro_rules! try_trace_now { ($d($arg:tt)+) => ( try_trace!(target: "!", $d($arg)+) ); }

                #[macro_use] #[macro_export]
                #[doc(hidden)]
                /// Fallible immediate version of `warn!`.
                macro_rules! try_warn_now { ($d($arg:tt)+) => ( try_warn!(target: "!", $d($arg)+) ); }

            }
        }
    }
}
