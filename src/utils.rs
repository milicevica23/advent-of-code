use std::path::Path;
use std::fs::read_to_string;


pub fn read_string(path: &Path)->String{
    read_to_string(path).expect("2problem")
}