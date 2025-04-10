
fn main(){
    unsafe{
    std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();
    macros_logger::my_info!("simple logging");
    macros_logger::my_info!(
        "dsl logging with args {}" => {"1"},
        "dsl logging with args {} {}" => {2, 1},
        "dsl logging with args {} {}" => {2, 2},
    );
}
