use core::fmt::Debug;

#[macro_export]
macro_rules! delog_panic {
    ($($arg:tt)*) => {{
        error_now!($($arg)*);
        panic!();
    }};
}

pub trait DelogPanic<T> {
    fn delog_unwrap(self) -> T;
    fn delog_expect(self, msg: &str) -> T;
}

impl<T> DelogPanic<T> for Option<T> {
    #[inline(always)]
    fn delog_unwrap(self) -> T {
        match self {
            Some(t) => t,
            None => delog_panic!("Called `Option::delog_unwrap` on a `None` value"),
        }
    }

    #[inline(always)]
    fn delog_expect(self, _msg: &str) -> T {
        match self {
            Some(t) => t,
            None => delog_panic!("{_msg}"),
        }
    }
}

impl<T, E: Debug> DelogPanic<T> for Result<T, E> {
    #[inline(always)]
    fn delog_unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(_e) => delog_panic!("Called `Result::delog_unwrap` on an `Err` value: {_e:?}"),
        }
    }

    #[inline(always)]
    fn delog_expect(self, _msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_e) => delog_panic!("{_msg}: {_e:?}"),
        }
    }
}
