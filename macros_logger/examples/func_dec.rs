use macros_logger::log_func;

log_func! {

fn say_hello(){
    println!("hello decorator");
}

}

fn main() {
    say_hello();
}
