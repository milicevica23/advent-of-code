#![allow(unused)]
use std::fmt;

use regex::Regex;


#[derive(Debug)]
pub enum Method{
    Plus, 
    Multiple,
    SelfMultiple,
    SelfPlus
}
#[derive(Debug)]
pub struct Monkey{
    pub id: i32,
    pub items_observed: i32,
    pub items: Vec<i32>,
    pub opertation_method: Method,
    pub operation_scalar: i32,
    pub test_scalar: i32,
    pub test_true: usize, 
    pub test_false: usize,
    total_count: usize
}

struct Change {
    id: usize,
    item: i32
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkey id: {}",self.id)
    }
}

pub fn parse_monkies(puzzle_input: &str) -> Vec<Monkey> {

    let re = Regex::new(
        r#"(?s)Monkey (?P<id>[0-9]+):\n*\s*Starting items: (?P<items>[0-9, ]*)\s*\n*\s*Operation: new = old (?P<op>\+|\*) (?P<op_sc>([0-9]*|old))\s*\n*\s*Test: divisible by (?P<test_sc>[0-9]*)\s*\n*\s*If true: throw to monkey (?P<true_to>[0-9]*)\s*\n*\s*If false: throw to monkey (?P<false_to>[0-9]*)"#
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
                opertation_method: {
                    match cap.name("op_sc").expect("should be an op_sc!").as_str() {
                    "old" => cap.name("op")
                    .map_or(Method::Plus, |m| 
                        match m.as_str() {
                            "*" => Method::SelfMultiple,
                            "+" => Method::SelfPlus,
                            op => panic!("Not found operation {} ? ",{op})
                        }
                    ),
                    _ => 
                    cap.name("op")
                    .map_or(Method::Plus, |m| 
                        match m.as_str() {
                            "*" => Method::Multiple,
                            "+" => Method::Plus,
                            op => panic!("Not found operation {} ? ",{op})
                        }
                    )
                }
            }   ,
                operation_scalar: cap.name("op_sc")
                .map_or(-1, |m| match m.as_str(){
                     "old" => -1,
                     a => a.parse::<i32>()
                     .expect("op_sc should be a valid int!")   
                }),
                test_scalar: cap.name("test_sc")
                .map_or(-1, |m| m.as_str()
                                                    .parse::<i32>()
                                                    .expect("test_sc should be a valid int!")),
                test_true: cap.name("true_to")
                .map_or(0, |m| m.as_str()
                                                    .parse::<usize>()
                                                    .expect("true_to should be a valid int!")),
                test_false: cap.name("false_to")
                .map_or(0, |m| m.as_str()
                                                    .parse::<usize>()
                                                    .expect("false_to should be a valid int!")),
                total_count: 0
            })
        }
    ).collect::<Vec<Monkey>>();
    result
}

pub fn play_game(input: &str) -> () {
    let mut monkeys = parse_monkies(input);
    let number_of_rounds = 10000;
    for round in 0..number_of_rounds {
        play_one_round(&mut monkeys);
    }
    let mut most_active_id :i32 = 0; 
    let mut most_active :usize = 0; 
    monkeys.iter().for_each(
        |m| if m.total_count > most_active && most_active_id != m.id {
            most_active = m.total_count;
            most_active_id = m.id
        }
    );

    let mut second_active_id :i32 = 0; 
    let mut second_active :usize = 0; 
    monkeys.iter().for_each(
        |m| if m.total_count > second_active && m.id !=most_active_id  {
            second_active = m.total_count;
            second_active_id = m.id
        }
    );
    let result = second_active * most_active;
    println!("{}", result)
}




fn play_one_round(monkeys: &mut Vec<Monkey>) {
    for each_monkey_id in 0..monkeys.len() {
        let monkey = monkeys.get(each_monkey_id).expect("monkey should exist!");
        let monkey_items = &monkey.items;
        let mut new_items:Vec<Change> = Vec::new();
        monkey_items.iter().for_each(
            |each_item| {
                
                let new_item_score = match monkey.opertation_method {
                    Method::Multiple => (each_item * monkey.operation_scalar) % monkey.test_scalar,
                    Method::Plus => each_item + monkey.operation_scalar,
                    Method::SelfMultiple => (each_item * each_item) % monkey.test_scalar ,
                    Method::SelfPlus => each_item + each_item,
                };
                let new_item_score = new_item_score;
                if (new_item_score % monkey.test_scalar) == 0 {
                    new_items.push(Change{id: monkey.test_true, item:new_item_score});
                }else {
                    new_items.push(Change{id: monkey.test_false, item:new_item_score});
                }
            }
        );
        let total_order = monkeys.get_mut(each_monkey_id).expect("monkey should exist!").items.len();
        monkeys.get_mut(each_monkey_id).expect("monkey should exist!").total_count+=total_order;
        monkeys.get_mut(each_monkey_id).expect("monkey should exist!").items.clear();

        for change in new_items.iter(){
            monkeys.get_mut(change.id).expect("monkey should exist!").items.push(change.item);
        }
    }
    
}


#[cfg(test)]
mod tests {
    use std::string;

    use super::*;
/*
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
    */
    #[test]
    fn test_game(){
        let monkey_text = 
        r#"Monkey 0:
        Starting items: 66, 59, 64, 51
        Operation: new = old * 3
        Test: divisible by 2
          If true: throw to monkey 1
          If false: throw to monkey 4
      
      Monkey 1:
        Starting items: 67, 61
        Operation: new = old * 19
        Test: divisible by 7
          If true: throw to monkey 3
          If false: throw to monkey 5
      
      Monkey 2:
        Starting items: 86, 93, 80, 70, 71, 81, 56
        Operation: new = old + 2
        Test: divisible by 11
          If true: throw to monkey 4
          If false: throw to monkey 0
      
      Monkey 3:
        Starting items: 94
        Operation: new = old * old
        Test: divisible by 19
          If true: throw to monkey 7
          If false: throw to monkey 6
      
      Monkey 4:
        Starting items: 71, 92, 64
        Operation: new = old + 8
        Test: divisible by 3
          If true: throw to monkey 5
          If false: throw to monkey 1
      
      Monkey 5:
        Starting items: 58, 81, 92, 75, 56
        Operation: new = old + 6
        Test: divisible by 5
          If true: throw to monkey 3
          If false: throw to monkey 6
      
      Monkey 6:
        Starting items: 82, 98, 77, 94, 86, 81
        Operation: new = old + 7
        Test: divisible by 17
          If true: throw to monkey 7
          If false: throw to monkey 2
      
      Monkey 7:
        Starting items: 54, 95, 70, 93, 88, 93, 63, 50
        Operation: new = old + 4
        Test: divisible by 13
          If true: throw to monkey 2
          If false: throw to monkey 0"#;
         let monkey_text = r#"Monkey 0:
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
          play_game(monkey_text);
        //21346465600
    }


}