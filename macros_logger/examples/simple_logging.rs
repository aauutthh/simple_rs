use macros_logger::dsl_info;

fn main(){
    unsafe{
    std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();
    macros_logger::simple_info!("simple logging");
    macros_logger::simple_info!("simple logging with args {}", 1);
    // dsl_info 宏有递归实现，必须use 到当前mod
    // macros_logger::dsl_info!(...) 会失败
    dsl_info!(
        "dsl logging with no args",
        "dsl logging with no args" => {},
        "dsl logging with args {}" => {"1"},
        "dsl logging with args {} {}" => {2, 1},
        "dsl logging with args {} {}" => {2, 2},
    );
}
