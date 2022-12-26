use std::collections::{HashMap, HashSet};


#[derive(Debug)]
struct Rucksag{
    firts_part: String,
    second_part: String,
    common: String
}

struct RucksagGroup{
    content: String,
    key: char
}

fn parse_ruck_part_2(input: &String) -> Vec<RucksagGroup>{
    let mut rucks = input.split("\n").collect::<Vec<_>>();
    rucks.push("last");
    let mut acc = String::from("");
    let mut group_rucks:Vec<RucksagGroup> = Vec::new();
    for (i, each_ruck) in rucks.iter().enumerate() {
        if i!=0 && i % 3 == 0 {
            let char_counts = 
                acc.chars().fold(HashMap::new(), 
            |mut map, each_char| {
                *map.entry(each_char).or_insert(0) +=1;
                 map
            }
            );
            group_rucks.push(
                RucksagGroup {
                    content: acc.clone(),
                    key: *char_counts.iter()
                        .find(|each_par| *each_par.1 == 3)
                        .expect("Should be at least one with 3 elements").0
                } 
            );
            acc.clear();
        }
        let mut unique_chars: String = String::from("");
        for each_char in each_ruck.chars() {
            if !unique_chars.contains(each_char) {
                unique_chars.push(each_char);
            }
        }
        acc.push_str(&unique_chars);
    }
    group_rucks

}


fn parse_ruck(input: &String)-> Vec<Rucksag> {
    let rucks = input.split("\n").collect::<Vec<_>>();

    rucks.iter().map(
        |each_line| {
            let half = each_line.len()/2;
            let mut firts_part = String::from("");
            let mut second_part= String::from("");
            let mut common= String::from("");
            for (i, c) in each_line.chars().enumerate() {
                if i < half {
                    firts_part.push(c);
                }else {
                    second_part.push(c);
                    if firts_part.contains(c) && !common.contains(c) {
                        common.push(c);
                    }
                }
            }
            Rucksag { firts_part, second_part, common}
        }
    ).collect::<Vec<_>>()
}

fn get_value(ch: char) -> u32 {
    if !ch.is_ascii() {
        panic!("{:?}", ch)
    };
    if ch.is_ascii_lowercase() {
        (ch as u32) - 96
    } else {
        (ch as u32) - 64 + 26
    }
}

#[cfg(test)]
mod test {
    use std::{fs, result};
    use super::*;


    #[test]
    fn play_game_part_2(){
        let base_path = std::env::current_dir().expect("not a valid dir!");
        //let file_path =  "C:\\Users\\SFP1Z6L\\git\\advent-of-code\\puzzle_input\\2022\\02\\description.txt";
        let file_path =  
                            base_path.to_str().expect("not a valid string!").to_owned() 
                            + "\\puzzle_input\\2022\\03\\submit.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

        let rucks = parse_ruck_part_2(&contents);
        
        let result =  rucks.iter().fold(0, 
            |acc, each_ruck|
            acc + get_value(each_ruck.key)
        );
        println!("{:?}", result);//.to_digit(10).expect("not a valid char"));
    }




    #[test]
    fn play_game(){
        let base_path = std::env::current_dir().expect("not a valid dir!");
        //let file_path =  "C:\\Users\\SFP1Z6L\\git\\advent-of-code\\puzzle_input\\2022\\02\\description.txt";
        let file_path =  
                            base_path.to_str().expect("not a valid string!").to_owned() 
                            + "\\puzzle_input\\2022\\03\\submit.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

        let rucks = parse_ruck(&contents);
        //println!("{:?}", contents);
        let result = rucks.iter().fold(0, 
        |acc, each_ruck| 
            acc + each_ruck.common.chars().fold(0, 
                    |ruck_acc,each_comm | ruck_acc + get_value(each_comm)
                ) 
        );

        println!("{:?}", result);//.to_digit(10).expect("not a valid char"));
    }

    #[test]
    fn is_correct_asci_value(){
        let result = get_value('Z'); 
        println!("{:?}", result);
    }
}