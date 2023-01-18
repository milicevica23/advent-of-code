

enum Move {
    Left,
    Right,
}
struct Game {
    current_rock:  Box<dyn Rock>,
    moves: Vec<Move>,
    x: i32, //breite
    max_y: i32
}


trait Rock {
    fn move_left(&self);
    fn move_right(&self);
    fn move_down(&self);    
}

struct MinusRock{
    parts: Vec<i32>,
}

impl MinusRock {
    pub fn new(start_position: i32, x_length: i32) -> Self {
        Self { 
            parts: vec![start_position,start_position+1,start_position+2,start_position+3],
        }
    }
}


impl Rock for MinusRock {
    fn move_left(&self){}
    fn move_right(&self){}
    fn move_down(&self){}
}


fn parse_game(puzzle_input: &str) -> Vec<Move> {
    puzzle_input.chars().map(
        |c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            ch => panic!("{} is not allowed", ch)
        }
    ).collect()
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
                            + "\\puzzle_input\\2022\\03\\submit.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    }

}