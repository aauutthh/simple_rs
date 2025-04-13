extern crate macros_function_decorator;
use macros_function_decorator::{log_call, with_logging};

#[log_call]
fn say_hello() {
    println!("hello decorator");
}

#[with_logging(level = "info", message = "hello decorator")]
fn say_hello2() {
    println!("hello decorator");
}

fn main() {
    say_hello();
    say_hello2();
}
