#![allow(unused)]
use std::{path::Path};
use crate::utils::read_string;
use crate::puzzle::monkey_in_the_middle_11::*;
use regex::Regex;
mod utils;
pub mod puzzle;

fn main() {
    /*let path = Path::new("./puzzle_input/2022/11/description_input.txt");
    let puzzle_input = read_string(path);
    println!("{}", puzzle_input);*/
    let puzzle_input = 
r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1"#;

    let re = Regex::new(
r#"(?s)Monkey (?P<id>[0-9]+):\n*\s*Starting items: (?P<items>[0-9, ]*)\s*\n*\s*Operation: new = old (?P<op>\+|\*) (?P<op_sc>[0-9]*)\s*\n*\s*Test: divisible by (?P<test_sc>[0-9]*)\s*\n*\s*If true: throw to monkey (?P<true_to>[0-9]*)\s*\n*\s*If false: throw to monkey (?P<false_to>[0-9]*)"#
    ).expect("should be a valid regex!");

    let result = re.captures_iter(puzzle_input).filter_map(|cap| {
        Some(Monkey {
                id: cap.name("id")
                    .map_or(-1, |m| m.as_str()
                                                       .parse::<i32>()
                                                       .expect("id should be a valid int!")),
                items_observed: 0,
                items: cap.name("items")
                           .map_or(vec![], |m| 
                                m.as_str().split(",")
                                    .map(|each_item| each_item.trim().parse::<i32>()
                                    .expect("can't parse an in items!"))
                           .collect::<Vec<i32>>()),
                opertation_method: cap.name("op")
                .map_or(Method::Plus, |m| match m.as_str() {
                    "*" => Method::Multiple,
                    "+" => Method::Plus,
                     op => panic!("Not found operation {} ? ",{op})
                }),
                operation_scalar: cap.name("op_sc")
                .map_or(-1, |m| m.as_str()
                                                   .parse::<i32>()
                                                   .expect("op_sc should be a valid int!")),
                test_scalar: cap.name("test_sc")
                .map_or(-1, |m| m.as_str()
                                                   .parse::<i32>()
                                                   .expect("test_sc should be a valid int!")),
                test_true: cap.name("true_to")
                .map_or(-1, |m| m.as_str()
                                                   .parse::<i32>()
                                                   .expect("true_to should be a valid int!")),
                test_false: cap.name("false_to")
                .map_or(-1, |m| m.as_str()
                                                   .parse::<i32>()
                                                   .expect("false_to should be a valid int!")),
            })
        }
    ).collect::<Vec<Monkey>>();
    println!("{:#?}", result);
}
