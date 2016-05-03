//! GDC Client Macros

#[macro_export]
macro_rules! error_and_exit {
    ($msg:expr,  $($arg:tt)*) => {{
        error!($msg, $($arg)*);
        exit(1);
    }};
    ($msg:expr) => {{
        error!($msg);
        exit(1);
    }}
}
