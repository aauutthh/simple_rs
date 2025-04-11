use mlogger::dsl_info;

fn main(){
    unsafe{
    std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();
    mlogger::simple_info!("simple logging");
    mlogger::simple_info!("simple logging with args {}", 1);
    dsl_info!(
        "dsl logging with no args",
        "dsl logging with no args" => {},
        "dsl logging with args {}" => {"1"},
        "dsl logging with args {} {}" => {2, 1},
        "dsl logging with args {} {}" => {2, 2},
    );
}
