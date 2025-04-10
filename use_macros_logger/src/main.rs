
fn main(){
    unsafe{
    std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();
    macros_logger::my_info!("simple logging");
    macros_logger::my_info!("simple logging with args {}" => {"<null>"});
}
