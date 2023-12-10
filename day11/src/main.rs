use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

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

fn check_row_empty(input: &String) -> bool {
    !input.contains('#')
}

fn check_column_empty(input: &Vec<String>, column: usize) -> bool {
    for s in input.iter() {
        if s.chars().nth(column).unwrap() == '#' {
            return false;
        }
    }
    true
}

fn expand_space(input: &mut Vec<String>) {
    let mut expanding_rows: Vec<usize> = Vec::new();
    let mut expanding_columns: Vec<usize> = Vec::new();

    for row in 0..input.len() {
        if check_row_empty(&input[row]) {
            expanding_rows.push(row);
        }
    }
    for column in 0..input[0].len() {
        if check_column_empty(input, column) {
            expanding_columns.push(column);
        }
    }
    for (i, column) in expanding_columns.iter().enumerate() {
        //eprintln!("column {} expands", column);

        for row in input.iter_mut() {
            *row = row[0..*column + i].to_string() + &"." + &row[*column + i..row.len()];
        }
    }

    for (i, row) in expanding_rows.iter().enumerate() {
        //eprintln!("Row {} expands", row);
        let new_row = [std::iter::repeat(".")
            .take(input[0].len())
            .collect::<String>()];
        input.splice(*row + i..*row + i, new_row.iter().cloned());
    }
}

fn get_galaxy_positions(input: &Vec<String>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y].chars().nth(x).unwrap() == '#' {
                res.push((x, y));
            }
        }
    }
    res
}

fn _print_space(space: &Vec<String>) {
    for row in space.iter() {
        println!("{}", row);
    }
    //println!("");
}

fn _replace_with_num(grid: &mut Vec<String>, galaxy_positions: &Vec<(usize, usize)>) {
    for (i, position) in galaxy_positions.iter().enumerate() {
        grid[position.1].replace_range(position.0..position.0 + 1, (1 + i).to_string().as_str());
    }
}

fn get_distance(a: &(usize, usize), b: &(usize, usize), _i: usize, _j: usize) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn make_pos_key(a: &(usize, usize), b: &(usize, usize)) -> String {
    if a.0 < b.0 {
        std::format!("{},{}-{},{}", a.0, b.0, a.1, b.1)
    } else if a.0 == b.0 && a.1 < b.1 {
        std::format!("{},{}-{},{}", a.0, b.0, a.1, b.1)
    } else {
        std::format!("{},{}-{},{}", b.0, a.0, b.1, a.1)
    }
}

/*
124,10 -> 53,19

-> 53,124-19,10

x0,y0,x1,y1

x0 < x1:
x0,x1-y0,y1
x0 > x1:
x1,x0-y1,y0

0,10 -> 5,10 => 0,5-10,10
3,10 -> 3,2 => 3,3-10,2
3,2 -> 3,10 => 3,3-2,10

*/
fn main() {
    let mut input: Vec<String> = parse_inputfile(&get_filename());
    expand_space(&mut input);
    let galaxy_positions: Vec<(usize, usize)> = get_galaxy_positions(&input);

    let mut distance_sum = 0;
    let mut star = 0;
    let mut galaxymap: HashMap<String, usize> = HashMap::new();

    for a in galaxy_positions.iter() {
        star += 1;
        let mut star_b = 0;
        for b in galaxy_positions.iter() {
            let key = make_pos_key(a, b);
            if a.0 != b.0 || a.1 != b.1 {
                galaxymap.insert(key, get_distance(a, b, star, star_b));
                star_b += 1;
            }
        }
        //println!("Checked len: {}", );
    }

    galaxymap.values().for_each(|x| distance_sum += x);
    println!("{}", distance_sum);
}
