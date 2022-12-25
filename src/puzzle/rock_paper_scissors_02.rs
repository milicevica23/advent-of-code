


#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn play_game(){
        let file_path =  "C:\\Users\\SFP1Z6L\\git\\advent-of-code\\puzzle_input\\2022\\02\\description.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

        
    }

}