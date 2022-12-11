use std::{path::Path};
use crate::utils::read_string;

mod utils;
mod puzzle;

fn main() {
    let path = Path::new("./puzzle_input/2022/11/description_input.txt");
    let puzzle_input = read_string(path);
    println!("{}", puzzle_input);
}
