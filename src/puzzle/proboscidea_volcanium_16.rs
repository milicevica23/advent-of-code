use std::{vec, collections::HashMap, hash::Hash};

use regex::Regex;


#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    goes_to: Vec<String>,
}

fn parse_valves(input: &String) -> Vec<Valve>{
    let re = Regex::new(
        r#"Valve (?P<name>[A-Z]{2}) has flow rate=(?P<flow_rate>[0-9]+); tunnel[s]* lead[s]* to valve[s]* (?P<goes_to>[A-Z, ]+)\n"#
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


#[derive(Debug)]
struct OneStep {
    current_move: String,
    release: bool,
    step_release:i32,
    total_release: i32,
    steps: Vec<String>,
}

fn play_game(input: Vec<Valve>) -> i32 {

    let mapping_move:HashMap<String, &Vec<String>> = input.iter().map(
        |each_valve| {
            (each_valve.name.clone(), &each_valve.goes_to)
        }
    ).collect();

    let mapping_release:HashMap<String, i32> = input.iter().map(
        |each_valve| {
            (each_valve.name.clone(), each_valve.flow_rate)
        }
    ).collect();


    let mut old_minute_step = Vec::new();
    old_minute_step.push(
        OneStep {
            current_move: String::from("AA"),
            release: false,
            step_release:0,
            total_release: 0,
            steps: vec![String::from("AA")],
        }
    );

    let mut new_minute_step = Vec::new();
    for i in (0..30) {
        for each_step in &old_minute_step {
            let move_release = *mapping_release.get(&each_step.current_move).unwrap();
            if each_step.release == false && move_release != 0 {
                let mut new_steps = each_step.steps.clone();
                new_steps.push( format!("{}-{}","O".to_owned(), each_step.current_move.clone()));
                new_minute_step.push(
                    OneStep{
                        current_move: each_step.current_move.clone(),
                        release: true,
                        step_release: each_step.step_release + move_release,
                        total_release: each_step.total_release + each_step.step_release,
                        steps: new_steps,
                    }
                )
            }
            let new_steps = *mapping_move.get(&each_step.current_move).unwrap();
            for possible_step in new_steps {
                let mut next_new_steps = each_step.steps.clone();
                if !next_new_steps.contains(possible_step) {
                    next_new_steps.push(possible_step.to_owned());
                    new_minute_step.push(
                    OneStep{
                        current_move: possible_step.to_owned(),
                        release: false,                        
                        step_release: each_step.step_release,
                        total_release: each_step.total_release + each_step.step_release,
                        steps: next_new_steps,
                    }
                    );
                }
            }
        }
        if new_minute_step.len() != 0 {
            old_minute_step.clear();
            old_minute_step = new_minute_step;
        }else {
            let old_minute_step = old_minute_step.iter().map(
                |each_step| {
                    OneStep {
                        total_release: each_step.total_release + each_step.step_release,  
                        current_move: each_step.current_move.to_owned(),
                        release: false,
                        step_release: each_step.step_release,
                        steps: each_step.steps.clone(),
                    }
                }
            );
        }
        new_minute_step = Vec::new();
    }

    let max_step = old_minute_step.iter().max_by_key(|each_step| each_step.total_release).unwrap();

    return max_step.total_release;

}






fn play_game_2(input: Vec<Valve>) -> i32 {
    let mapping_move:HashMap<String, &Vec<String>> = input.iter().map(
        |each_valve| {
            (each_valve.name.clone(), &each_valve.goes_to)
        }
    ).collect();

    let mapping_release:HashMap<String, i32> = input.iter().map(
        |each_valve| {
            (each_valve.name.clone(), each_valve.flow_rate)
        }
    ).collect();

    let mut round_left = 30;
    let mut my_position = "AA".to_owned();
    let already_seen = vec![];

    loop {
        let cost_maping = find_how_many_steps(
            mapping_move,
            my_position,
        );

        let max_position = cost_maping.iter().max_by_key(
            |(key, steps_away)| {
                let release = *mapping_release.get(*key).unwrap();
                let round_till_end = round_left - *steps_away - 1; 
                if round_till_end >= 0 {
                    round_till_end * release
                }else{
                    -1
                }
            }
        ).unwrap();




    }


    return 0;
}

fn find_how_many_steps(mapping_move:HashMap<String, &Vec<String>>,
                       start_position: String
                    ) -> HashMap<String, i32>{
    let mut steps_mapping = HashMap::new();
    let all_keys = mapping_move.keys();
    
    for element in all_keys{
        if *element == start_position {
            steps_mapping.insert(element.to_owned(), 0);
        }
        let mut step = 1; 
        let already_seen = Vec::new();
        let mut current_step = start_position.clone();
        let new_elements = *mapping_move.get(&current_step).unwrap();
        loop {
            if new_elements.contains(&element) {
                steps_mapping.insert(element.to_owned(),step);
                break;
            }
            let mut next_step_elements = Vec::new();
            for each_element in new_elements{
                if !already_seen.contains(each_element){
                    let new = *mapping_move.get(each_element).unwrap();
                    for each in new {
                        if !already_seen.contains(each){
                            next_step_elements.push(each.to_owned());
                        }
                    }
                    already_seen.push(each_element.to_owned());
                } 
            }
            new_elements.append(&mut next_step_elements);
            step +=1;
        }
    }
    return steps_mapping;
}


#[cfg(test)]
mod test {
    use std::{fs, result};
    use super::*;


    #[test]
    fn play_game_1(){
        let base_path = std::env::current_dir().expect("not a valid dir!");
        //let file_path =  "C:\\Users\\SFP1Z6L\\git\\advent-of-code\\puzzle_input\\2022\\02\\description.txt";
        let file_path =  
                            base_path.to_str().expect("not a valid string!").to_owned() 
                            + "\\puzzle_input\\2022\\16\\description.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

        let puzzle = parse_valves(&contents);

        let result = play_game(puzzle);
        println!("{:#?}", result);
    }

}