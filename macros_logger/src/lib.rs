
#[macro_export]
macro_rules! my_info{
    ($fmt:expr, $($args:tt)+) => {{
        log::info!($fmt, $($args)+);
    }};
    ($fmt:expr) => {{
        log::info!($fmt);
    }};
}
