#![allow(unused)]
use std::{path::Path};
use crate::utils::read_string;
use regex::Regex;
mod utils;
mod puzzle;

fn main() {
    /*let path = Path::new("./puzzle_input/2022/11/description_input.txt");
    let puzzle_input = read_string(path);
    println!("{}", puzzle_input);*/
    let puzzle_input = r#"Monkey 12:"#;
    let re = Regex::new(
        r#"Monkey (\d+):"#
    ).expect("should be a valid regex!");
    let result = re.captures(puzzle_input).unwrap();
    let a = 5;
    //println!("{}", result.())
}
