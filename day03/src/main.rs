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
            result.push(format!("{}.", line.to_string().trim()));
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

fn is_num(c: char) -> bool {
    return (c >= '0') & (c <= '9');
}

/// Parse number and length of number from String beginning with a number.
fn parse_num(input: &String) -> (i32, usize) {
    let mut end: usize = 0;
    for c in input.chars() {
        if is_num(c) {
            end += 1;
        } else {
            break;
        }
    }

    (input[..end].to_string().parse().unwrap(), end as usize)
}

/// value: Positive number for number, 0 for ., -1 for anything else.
struct Entry {
    value: i32,
    num_beginning: usize,
    num_len: usize,
    used: bool,
}

fn inputline_to_struct_vec(input: &String) -> Vec<Entry> {
    let mut is_num_c: usize = 0;
    let mut ret: Vec<Entry> = Vec::new();

    for (i, c) in input.chars().enumerate() {
        if is_num_c > 0 {
            assert!(is_num(c));
            is_num_c -= 1;
            continue;
        }

        if is_num(c) {
            let res: (i32, usize) = parse_num(&input[i..].to_string());
            is_num_c = res.1;
            let val: i32 = res.0;
            assert_eq!(val.to_string().len(), is_num_c);

            for x in 0..is_num_c {
                ret.push(Entry {
                    value: val.clone(),
                    num_beginning: x,
                    num_len: is_num_c,
                    used: false,
                });
            }
            is_num_c -= 1;
        } else {
            if c == '.' {
                ret.push(Entry {
                    value: 0,
                    num_beginning: 0,
                    num_len: 0,
                    used: false,
                });
            } else {
                let val = if c == '*' { -2 } else { -1 };
                ret.push(Entry {
                    value: val,
                    num_beginning: 0,
                    num_len: 0,
                    used: false,
                });
            }
        }
    }
    ret
}

/// Mark number as already used
fn mark_used(grid: &mut Vec<Vec<Entry>>, x: usize, y: usize) {
    let num_first_digit_x = x - grid[y][x].num_beginning;

    for i in num_first_digit_x..num_first_digit_x + grid[y][x].num_len {
        grid[y][i].used = true;
    }
}

fn get_all_nearby(grid: &mut Vec<Vec<Entry>>, x: usize, y: usize) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    let mut was_else = false;

    let b_x_diff: usize = 1;
    let e_x_diff: usize = 2;

    let b_x: usize = if x >= b_x_diff { x - b_x_diff } else { 0 };
    let e_x: usize = if x + e_x_diff <= grid[0].len() {
        x + e_x_diff
    } else {
        was_else = true;
        grid[0].len()
    };

    let b_y_diff: usize = 1;
    let e_y_diff: usize = 2;

    let b_y: usize = if y >= b_y_diff { y - b_y_diff } else { 0 };
    let e_y: usize = if y + e_y_diff <= grid.len() {
        y + e_y_diff
    } else {
        was_else = true;
        grid.len()
    };
    let mut iters = 0;
    for y_i in b_y..e_y {
        for x_i in b_x..e_x {
            iters += 1;
            if (grid[y_i][x_i].value > 0) & (!grid[y_i][x_i].used) {
                mark_used(grid, x_i, y_i);
                res.push(grid[y_i][x_i].value as u32);
            }
        }
    }
    if !was_else {
        assert_eq!(iters, 9);
    }
    res
}

/// Check if position x, y is unused number next to non dot symbol.
/// Return value of symbol if so.
fn check_is_next_to_num(grid: &mut Vec<Vec<Entry>>, x: usize, y: usize, multiply: bool) -> u32 {
    if grid[y][x].value >= 0 {
        return 0;
    }

    let mut sum: u32 = 0;

    let nearby: Vec<u32> = get_all_nearby(grid, x, y);

    if (grid[y][x].value == -2) & (nearby.len() == 2) & multiply {
        sum += nearby[0] * nearby[1];
    } else if !multiply {
        for val in nearby.iter() {
            sum += val;
        }
    }

    sum
}

fn parse_to_struct(input: &Vec<String>) -> Vec<Vec<Entry>> {
    let mut input_structs: Vec<Vec<Entry>> = Vec::new();
    for line in input.iter() {
        input_structs.push(inputline_to_struct_vec(line));
    }
    input_structs
}

fn main() {
    let input: Vec<String> = parse_inputfile(&get_filename());
    let mut input_structs: Vec<Vec<Entry>> = parse_to_struct(&input);
    let mut sum = 0;

    for y in 0..input_structs.len() {
        for x in 0..input_structs[0].len() {
            sum += check_is_next_to_num(&mut input_structs, x, y, false);
        }
    }

    input_structs = parse_to_struct(&input);

    let mut sum_gears = 0;
    for y in 0..input_structs.len() {
        for x in 0..input_structs[0].len() {
            sum_gears += check_is_next_to_num(&mut input_structs, x, y, true);
        }
    }

    println!("{} {}", sum, sum_gears);
}
