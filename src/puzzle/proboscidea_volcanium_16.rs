use regex::Regex;


#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    goes_to: Vec<String>,
}

fn parse_valves(input: &String) -> Vec<Valve>{
    let re = Regex::new(
        r#"Valve (?P<name>[A-Z]{2}) has flow rate=(?P<flow_rate>[0-9]+); tunnels lead to valves (?P<goes_to>[A-Z, ]+)\n"#
            ).expect("should be a valid regex!");

    re.captures_iter(&input).map(
        |cap|
        Valve {
            name: cap.name("name").map_or(
                                    "No name",
                                        |m| m.as_str()
                                ).to_owned(),
            flow_rate: cap.name("flow_rate")
                        .map_or(-1, |m| m.as_str()
                                                .parse::<i32>()
                                                .expect("flow_rate should be a valid int!")),
            goes_to: cap.name("goes_to")
                    .map_or(vec![], |m| 
                        m.as_str().split(",")
                            .map(|each_item| each_item.trim().to_owned())
                    .collect::<Vec<String>>()),                                           
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
                            + "\\puzzle_input\\2022\\16\\description.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

        let puzzle = parse_valves(&contents);
        println!("{:#?}", puzzle);
    }

}