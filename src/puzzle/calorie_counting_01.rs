use std::fmt;

struct Elf {
    snacks : Vec<i32>
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Elf: {}", "test");
    }
}

fn play_game(){

}

fn read_elf_items(input: &str) -> Vec<Elf> {

    let elfs: Vec<String> = input.split("\n\n").collect::<String>();

    return elfs.map( |each_elf:String| 
            Elf{
                snacks : each_elf.split("\n").map(
                    |each_snack:str| each_snack.parse::<i32>()
                )        
            }
        )

}

#[cfg(test)] 
mod tests {
    use super::*;

    fn test_parser(){
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000";
        let result = read_elf_items(input);  
        println!("{}", result)
    }
}