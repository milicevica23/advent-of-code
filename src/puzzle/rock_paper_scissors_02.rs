
#[derive(Debug)]
enum Move{
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Game {
    oponent_move: Move,
    my_move: Move,
}

fn parse_game(input: &str) -> Vec<Game> {
    let rows = input.split("\n").collect::<Vec<_>>();

    rows.iter().map(
        |each_row| match *each_row {
            "A X" => Game {oponent_move: Move::Rock, my_move: Move::Rock},
            "A Y" => Game {oponent_move: Move::Rock, my_move: Move::Paper},
            "A Z" => Game {oponent_move: Move::Rock, my_move: Move::Scissors},
            "B X" => Game {oponent_move: Move::Paper, my_move: Move::Rock},
            "B Y" => Game {oponent_move: Move::Paper, my_move: Move::Paper},
            "B Z" => Game {oponent_move: Move::Paper, my_move: Move::Scissors},
            "C X" => Game {oponent_move: Move::Scissors, my_move: Move::Rock},
            "C Y" => Game {oponent_move: Move::Scissors, my_move: Move::Paper},
            "C Z" => Game {oponent_move: Move::Scissors, my_move: Move::Scissors},
            x => panic!("not a possible game: {}", x),
        }
    ).collect::<Vec<_>>()
}

fn calculate_reward(game: &Game) -> i32 {
    match game {
        Game {oponent_move: Move::Rock, my_move: Move::Rock} => 3 + 1,
        Game {oponent_move: Move::Rock, my_move: Move::Paper} => 6 + 2,
        Game {oponent_move: Move::Rock, my_move: Move::Scissors} => 0 + 3,
        Game {oponent_move: Move::Paper, my_move: Move::Rock} => 0 + 1,
        Game {oponent_move: Move::Paper, my_move: Move::Paper} => 3 + 2,
        Game {oponent_move: Move::Paper, my_move: Move::Scissors}=> 6 + 3, 
        Game {oponent_move: Move::Scissors, my_move: Move::Rock} => 6 + 1,
        Game {oponent_move: Move::Scissors, my_move: Move::Paper} => 0 + 2,
        Game {oponent_move: Move::Scissors, my_move: Move::Scissors} => 3 + 3 
    }
}

fn calculate_reward_part_2(game: &Game) -> i32 {
    match game {
        Game {oponent_move: Move::Rock, my_move: Move::Rock} => 3 + 1,
        Game {oponent_move: Move::Rock, my_move: Move::Paper} => 6 + 2,
        Game {oponent_move: Move::Rock, my_move: Move::Scissors} => 0 + 3,
        Game {oponent_move: Move::Paper, my_move: Move::Rock} => 0 + 1,
        Game {oponent_move: Move::Paper, my_move: Move::Paper} => 3 + 2,
        Game {oponent_move: Move::Paper, my_move: Move::Scissors}=> 6 + 3, 
        Game {oponent_move: Move::Scissors, my_move: Move::Rock} => 6 + 1,
        Game {oponent_move: Move::Scissors, my_move: Move::Paper} => 0 + 2,
        Game {oponent_move: Move::Scissors, my_move: Move::Scissors} => 3 + 3 
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn play_game(){
        let file_path =  "C:\\Users\\SFP1Z6L\\git\\advent-of-code\\puzzle_input\\2022\\02\\description.txt";
        //let file_path =  "C:\\Users\\SFP1Z6L\\git\\advent-of-code\\puzzle_input\\2022\\02\\submit.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
        
        let games = parse_game(&contents);

        let result = games.iter().fold(0, |i, g| i + calculate_reward(g));
        println!("{}", result)
    }

}