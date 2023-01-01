use std::io::BufRead;

#[derive(Debug)]
struct Pair {
    first_begin: i32,
    first_end: i32,
    second_begin: i32,
    second_end: i32,
}


fn parse_game(input: &str) -> Vec<Pair>{
    let pairs = input.split("\n").collect::<Vec<_>>();
    pairs.iter().map(
        |pair| {
            
            let pair_split = pair.split(",").collect::<Vec<_>>();
            let first = pair_split.get(0).unwrap().split("-").collect::<Vec<_>>();
            let second = pair_split.get(1).unwrap().split("-").collect::<Vec<_>>();
            Pair {
                first_begin: first.get(0).unwrap().trim().parse::<i32>().unwrap(),
                first_end: first.get(1).unwrap().trim().parse::<i32>().unwrap(),
                second_begin: second.get(0).unwrap().trim().parse::<i32>().unwrap(),
                second_end: second.get(1).unwrap().trim().parse::<i32>().unwrap(),
            }
        }
    ).collect::<Vec<_>>()
}
#[cfg(test)]
mod test {
    use std::{fs, result};
    use super::*;


    #[test]
    fn play_game(){
        let base_path = std::env::current_dir().expect("not a valid dir!");
        //let file_path =  "C:\\Users\\SFP1Z6L\\git\\advent-of-code\\puzzle_input\\2022\\02\\description.txt";
        let file_path =  
                            base_path.to_str().expect("not a valid string!").to_owned() 
                            + "\\puzzle_input\\2022\\04\\submit.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

        let result = parse_game(&contents);

        let result = result.iter().filter(
            |p| {
                (p.first_begin <= p.second_begin && p.first_end >= p.second_end)
                ||  
                (p.first_begin >= p.second_begin && p.first_end <= p.second_end)
            }
        ).collect::<Vec<_>>();

        println!("{}", result.len());

        let result = parse_game(&contents);

        let result = result.iter().filter(
            |p| {
                (p.first_begin <= p.second_end && p.first_begin >= p.second_begin)
                ||  
                (p.first_end <= p.second_end && p.first_end >= p.second_begin)
                ||
                (p.second_begin <= p.first_end && p.second_begin >= p.first_begin)
                ||  
                (p.second_end <= p.first_end && p.second_end >= p.first_begin)
            }
        ).collect::<Vec<_>>();

        println!("{}", result.len());
    }

}