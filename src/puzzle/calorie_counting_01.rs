use std::fmt;

struct Elf {
    snacks : Vec<i32>
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
                    |each_snack:&str| each_snack.parse::<i32>().expect("not a valid int!")
                ).collect()        
            }
        ).collect::<Vec<_>>()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
        
        10000".trim();
        let result = read_elf_items(input);  
    }
}