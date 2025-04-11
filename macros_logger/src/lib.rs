
#[macro_export]
macro_rules! simple_info{
    ($fmt:expr,$($args:tt)*) => {{
        log::info!($fmt, $($args)*);
    }};
    ($fmt:expr) => {{
        log::info!($fmt);
    }};
}

// 简单创建一种DSL
// my_info!(
//     "dsl logging with args {}" => {"1"},
//     "dsl logging with args {} {}" => {2, 1},
//     "dsl logging with args {} {}" => {2, 2},
// );
#[macro_export]
macro_rules! dsl_info {
    // 匹配纯字符串、带空参数、带参数的混合形式
    ($($fmt:expr $(=> {$($args:expr),*})?),* $(,)?) => {
        {
            $(
                // 这种实现方式可以直接使用 macros_logger::dsl_info!(...)
                // 但如果使用了 Cargo.toml 的crate 重命名, 则会失效
                // macros_logger::dsl_info!(@single $fmt $( => {$($args),*})?);


                // 这种实现方式必须 use macros_logger::dsl_info; 后才可以正常使用
                dsl_info!(@single $fmt $( => {$($args),*})?);
            )*
        }
    };

    // 无参
    (@single $fmt:expr) => {
        log::info!($fmt);
    };

    // 带参数（包括空参数）
    (@single $fmt:expr => {$($args:expr),*}) => {
        log::info!($fmt, $($args),*);
    };

}
