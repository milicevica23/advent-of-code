


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