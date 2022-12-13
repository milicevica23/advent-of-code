#![allow(unused)]
use std::fmt;

use regex::Regex;


#[derive(Debug)]
pub enum Method{
    Plus, 
    Multiple
}
#[derive(Debug)]
pub struct Monkey{
    pub id: i32,
    pub items_observed: i32,
    pub items: Vec<i32>,
    pub opertation_method: Method,
    pub operation_scalar: i32,
    pub test_scalar: i32,
    pub test_true: i32, 
    pub test_false: i32
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkey id: {}",self.id)
    }
}

pub fn parse_monkies(puzzle_input: &str) -> Vec<Monkey> {

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
    result
}

pub fn play_game(input: &str) -> () {
    let monkeys = parse_monkies(input);




}



#[cfg(test)]
mod tests {
    use std::string;

    use super::*;

    #[test]
    fn test_monkey_parser() {
        
        let monkey_text = 
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
                If false: throw to monkey 0"#;
        
        let result = parse_monkies(&monkey_text);
        assert_eq!(" Monkey id: 0 Monkey id: 1", 
            result.iter()
            .fold("".to_string(), |ac, m| format!("{} {}",ac, &m.to_string()))
        , "testing parser {} {}", "","");
    }
}