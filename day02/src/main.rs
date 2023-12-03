use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::option::Option;

struct Game {
    cube_sets: Vec<HashMap<String, u32>>,
    index: u32,
}

/// Read file filename into a vector, with each line as on element.
fn parse_inputfile(filename: &String) -> Vec<String> {
    if filename.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn get_filename() -> String {
    for (i, arg) in env::args().enumerate() {
        if i == 1 {
            return arg.to_string();
        }
    }
    String::new()
}

/// Get index of the game
fn parse_index(input: &String) -> Option<u32> {
    if !input.starts_with("Game ") {
        return None;
    }

    let end: Option<usize> = input.find(':');
    if end.is_some() {
        let indexstr = &input[5..end.unwrap()].to_string();

        return Some(indexstr.to_string().parse().unwrap());
    }

    None
}

fn get_next_or_end(input: &String, limiter: char) -> usize {
    let limiter_index: Option<usize> = input.find(limiter);

    if limiter_index.is_none() {
        return input.len();
    }

    limiter_index.unwrap()
}

/// Input should be in format 'count color,'
fn parse_color_count(input: &String) -> (String, u32) {
    let mut color: String = String::new();
    let mut count: u32 = 0;

    let parts: std::str::Split<'_, char> = input.split(' ');
    for (i, part) in parts.enumerate() {
        if i == 1 {
            color = part.to_string();
        } else {
            count = part.to_string().parse().unwrap();
        }
    }

    (color, count)
}

/// Parse one set of colors into a HashMap with color as the key
/// and count as the value
/// Input should be a string in format 1 blue, 2 red, 3 green
fn parse_set(input: &String) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();
    let parts = input.split(',');
    for part in parts {
        let output = parse_color_count(&part.to_string().trim().to_string());
        res.insert(output.0, output.1);
    }

    res
}

fn parse_sets(input: &String) -> Option<Vec<HashMap<String, u32>>> {
    let mut res: Vec<HashMap<String, u32>> = Vec::new();
    let mut start_index = get_next_or_end(input, ':') + 1;
    let mut end_index = start_index - 1;

    while start_index < input.len() {
        end_index += get_next_or_end(&input[start_index..].to_string(), ';') + 1;
        res.push(parse_set(&input[start_index + 1..end_index].to_string()));
        start_index = end_index + 1;
    }

    Some(res)
}

fn parse_game(input: &String) -> Option<Game> {
    let index = parse_index(input);
    let cube_sets = parse_sets(input);

    Some(Game {
        cube_sets: cube_sets.unwrap(),
        index: index.unwrap(),
    })
}

fn parse_games(input: Vec<String>) -> Vec<Game> {
    let mut res: Vec<Game> = Vec::new();
    for line in input.iter() {
        let parsed: Option<Game> = parse_game(line);
        if parsed.is_some() {
            res.push(parsed.unwrap());
        }
    }
    res
}

fn count_all(game: &Game) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();
    for set in game.cube_sets.iter() {
        for (key, value) in set.iter() {
            if res.contains_key(key) {
                res.insert(key.to_string(), value + res.get(key).unwrap());
            } else {
                res.insert(key.to_string(), value.clone());
            }
        }
    }
    res
}

fn count_max(game: &Game) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();

    for set in game.cube_sets.iter() {
        for (key, value) in set.iter() {
            if !res.contains_key(key) {
                res.insert(key.to_string(), value.clone());
            } else if res.get(key).unwrap() < value {
                res.insert(key.to_string(), value.clone());
            }
        }
    }
    res
}

fn get_color_count(input: &HashMap<String, u32>, color: String) -> u32 {
    let val: Option<&u32> = input.get(&color);
    if val.is_none() {
        return 0;
    }
    //println!("Color {} count: {}", color, val.unwrap());

    val.unwrap().clone()
}

fn _print_set(set: &HashMap<String, u32>) {
    for (key, val) in set {
        println!("{}: {}", key, val);
    }
}

fn _print_game(game: &Game) {
    println!("\n\nGame {}:", game.index);

    for set in game.cube_sets.iter() {
        print_set(set);
    }
}

fn main() {
    let lim_red = 12;
    let lim_green = 13;
    let lim_blue = 14;

    let input: Vec<String> = parse_inputfile(&get_filename());
    let games: Vec<Game> = parse_games(input);
    let mut impossible_sum: u32 = 0;
    let mut possible_sum: u32 = 0;
    let mut power_sum: u64 = 0;

    for game in games.iter() {
        //print_game(game);
        let max_cubes = count_max(game);
        let red = get_color_count(&max_cubes, "red".to_string());
        let blue = get_color_count(&max_cubes, "blue".to_string());
        let green = get_color_count(&max_cubes, "green".to_string());

        if (red > lim_red) | (blue > lim_blue) | (green > lim_green) {
            impossible_sum += game.index;
        } else {
            possible_sum += game.index;
        }
        let power = red * blue * green;
        power_sum += power as u64;
    }

    println!("Impossible sum: {}", impossible_sum);
    println!("Possible sum: {}", possible_sum);
    println!("Power sum: {}", power_sum);
}
