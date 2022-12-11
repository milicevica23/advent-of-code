use crate::utils::*; 
use console::Term;
use std::thread;
use std::time::Duration;

mod utils;

fn main() {
    let term = Term::stdout();
    term.write_line("Hello World!").expect("should write to terminal");
    thread::sleep(Duration::from_millis(2000));
    term.clear_line().expect("wrong");
    let text = "test";
    hello(text);
    println!("Hello, world! {}", text);
}
