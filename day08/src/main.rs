use num::complex::ComplexFloat;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use num::integer::lcm;

/// INPUT PARSES

/// Read file filename into a vector, with each line as on element.
fn parse_inputfile(filename: &String) -> Vec<String> {
    if filename.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        if !line.is_empty() {
            result.push(line.to_string().trim().to_string());
        }
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

enum Direction {
    Left,
    Right,
}

fn c2dir(c: char) -> Direction {
    if c == 'L' {
        return Direction::Left;
    }
    Direction::Right
}

#[derive(Clone)]
struct MapNode {
    left: String,
    right: String,
    me: String,
}

impl MapNode {
    fn new(left: String, right: String, me: String) -> MapNode {
        MapNode {
            left: left,
            right: right,
            me: me,
        }
    }

    fn from_line(line: &String) -> (String, MapNode) {
        let mut parts = line.split_whitespace();
        let name = parts.clone().nth(0).unwrap().to_string();

        let re: Regex = Regex::new("[a-zA-Z0-9]+").unwrap();

        let left_s = parts.clone().nth(2).unwrap().to_string();
        let right_s = parts.nth(3).unwrap().to_string();

        (
            name.clone(),
            MapNode::new(
                re.find(&left_s).unwrap().as_str().to_string(),
                re.find(&right_s).unwrap().as_str().to_string(),
                name,
            ),
        )
    }

    fn get_next(&self, dir: Direction) -> String {
        match dir {
            Direction::Left => return self.left.clone(),
            Direction::Right => {
                return self.right.clone();
            }
        }
    }

    fn navigate<'a>(&self, dir: Direction, map: &'a HashMap<String, MapNode>) -> &'a MapNode {
        map.get(&self.get_next(dir)).unwrap()
    }
}

fn main() {
    let input: Vec<String> = parse_inputfile(&get_filename());
    let instructions = input[0].clone();

    let parsing_start: std::time::Instant = std::time::Instant::now();

    let mut map: HashMap<String, MapNode> = HashMap::new();

    for line in input[1..input.len()].iter() {
        let val = MapNode::from_line(line);
        map.insert(val.0, val.1);
    }
    println!("Parsing took: {:?}", parsing_start.elapsed());

    let part1_start: std::time::Instant = std::time::Instant::now();

    let mut position: &MapNode = map.get(&"AAA".to_string()).unwrap();

    let mut steps = 0;

    while position.left != position.me
        && position.right != position.me
        && position.me != "ZZZ".to_string()
    {
        for instruction in instructions.chars() {
            position = position.navigate(c2dir(instruction), &map);
            steps += 1;
        }
    }
    println!("Part 1 took: {:?}, result {}", part1_start.elapsed(), steps);

    let part2_start: std::time::Instant = std::time::Instant::now();

    let mut positions: Vec<&MapNode> = Vec::new();
    for key in map.keys() {
        if key.ends_with('A') {
            positions.push(map.get(key).unwrap());
        }
    }

    let mut results = Vec::new();

    for (i, position_start) in positions.iter().enumerate() {
        steps = 0;
        let mut position = position_start.clone();
        while !position.me.ends_with('Z')
        {
            for instruction in instructions.chars() {
                position = position.navigate(c2dir(instruction), &map);
                steps += 1;
            }
        }
        println!("Ghost {} landed on {} after {} steps", i + 1, position.me, steps);
        results.push(steps);
    }
    let mut prev = results[0] as u64;

    for val in results.iter() {
        prev = lcm(prev, val.clone() as u64);

    }
    println!("Part 1 took: {:?}, result {}", part2_start.elapsed(), prev);

}
