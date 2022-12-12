#![allow(unused)]
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
   pub  test_scalar: i32,
    pub test_true: i32, 
    pub test_false: i32
}

pub fn parse_monkies(puzzle_input: &str) -> Vec<Monkey> {

    let re = Regex::new(
        r#""#
    ).expect("should be a valid regex!");

    let re = Regex::new(
        r#"Monkey (\d+):"#,
    ).expect("should be a valid regex!");
    let result = re.captures(puzzle_input).unwrap();
    

    /*let result = re.captures_iter(puzzle_input).filter_map(|cap| {
        let a = cap;
        Some(Monkey {
                id: 1,
                items_observed: 0,
                items: vec![1,2],
                opertation_method: Method::Plus,
                operation_scalar: 2,
                test_scalar: 4,
                test_true: 2, 
                test_false: 3
            })
        }
    ).collect::<Vec<Monkey>>();*/
    vec![]
}

pub fn play_game(input: String) -> () {

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_parser() {
        let a = 5;
        let b = a + 3; 
        let monkey_text = 
        r#"Monkey 0:
             Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
        "#;

        let monkey_text = 
        r#"Monkey 0:Monkey 0:"#;
        
        let result = parse_monkies(&monkey_text);
        //assert_eq!();
    }
}