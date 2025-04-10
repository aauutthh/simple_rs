
// 简单创建一种DSL
// my_info!(
//     "dsl logging with args {}" => {"1"},
//     "dsl logging with args {} {}" => {2, 1},
//     "dsl logging with args {} {}" => {2, 2},
// );
#[macro_export]
macro_rules! my_info{
    ($($fmt:expr => {$($args:expr),*}),* $(,)?) => {
        {
            $(
                log::info!($fmt, $($args),*);
            )*
        }
    };
    ($fmt:expr) => {{
        log::info!($fmt);
    }};
}
