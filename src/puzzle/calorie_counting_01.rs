use std::fmt;
#[derive(Debug)]
struct Elf {
    snacks : Vec<i32>
}

impl Elf {
    fn calculate_total(&self) -> i32 {
        self.snacks.iter().fold(0,|a, b| a + b)
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Elf: {}", "test")
    }
}

fn play_game(){

}

fn read_elf_items(input: &str) -> Vec<Elf> {

    let elfs: Vec<&str> = input.split("\n\n").collect();

    return elfs.iter().map( |each_elf | 
            Elf{
                snacks : each_elf.split("\n").map(
                    |each_snack:&str| {
                    //println!("{}", each_snack);
                    each_snack.trim().parse::<i32>().expect("not a valid int!")}
                ).collect()        
            }
        ).collect::<Vec<_>>()

}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parser(){
        let file_path =  "C:\\Users\\SFP1Z6L\\git\\advent-of-code\\puzzle_input\\2022\\01\\submit_input.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
        let mut result = read_elf_items(&contents);  

        let max_elf = result.iter().max_by_key(|a| a.calculate_total());
        println!("{}",max_elf.expect("max_elf").calculate_total());

        result.sort_by_key(|a| -a.calculate_total());

        println!("{:#?}", result);
        let max_three = result[0..3].iter().fold(0,|a, b| a + b.calculate_total());
        println!("max three: {}",max_three);
    }
}