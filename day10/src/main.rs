use std::env;
use std::fs::read_to_string;
use std::{thread, time};

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

struct Pos {
    x: usize,
    y: usize,
    p_x: usize,
    p_y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {
            x: x,
            y: y,
            p_x: x,
            p_y: y,
        }
    }

    fn update_prev_to_current(&mut self) {
        self.p_x = self.x;
        self.p_y = self.y;
    }

    // move left
    fn m_l(&mut self) {
        self.update_prev_to_current();
        self.x -= 1;
    }

    // move right
    fn m_r(&mut self) {
        self.update_prev_to_current();
        self.x += 1;
    }

    // move up
    fn m_u(&mut self) {
        self.update_prev_to_current();
        self.y -= 1;
    }

    // move down
    fn m_d(&mut self) {
        self.update_prev_to_current();
        self.y += 1;
    }

    fn step(&mut self, grid: &Grid) {
        match grid.get(self.x, self.y) {
            'L' => {
                if self.x < self.p_x {
                    self.m_u();
                } else {
                    self.m_r();
                }
            }

            '-' => {
                if self.p_x > self.x {
                    self.m_l();
                } else {
                    self.m_r();
                }
            }
            '|' => {
                if self.y < self.p_y {
                    self.m_u();
                } else {
                    self.m_d();
                }
            }

            'J' => {
                if self.y > self.p_y {
                    self.m_l();
                } else {
                    self.m_u();
                }
            }

            'F' => {
                if self.y < self.p_y {
                    self.m_r();
                } else {
                    self.m_d();
                }
            }

            '7' => {
                if self.x > self.p_x {
                    self.m_d();
                } else {
                    self.m_l();
                }
            }

            _ => {}
        }
    }

    fn print(&self, grid: &Grid) {
        println!("\n");
        for (y, row) in grid.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                print!(
                    "{}",
                    if (x == self.x) && (y == self.y) {
                        '*'
                    } else {
                        *c
                    }
                );
            }
            println!("");
        }
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn from_v_str(input: Vec<String>) -> Grid {
        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in input.iter() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        }
        Grid { grid: grid }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn get_start(&self) -> Pos {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.get(x, y) == 'S' {
                    return Pos::new(x, y);
                }
            }
        }

        Pos::new(0, 0)
    }
}

fn main() {
    let input: Vec<String> = parse_inputfile(&get_filename());
    let grid: Grid = Grid::from_v_str(input);
    let mut pos: Pos = grid.get_start();
    //pos.print(&grid);
    println!("Grid width: {}", grid.grid[0].len());
    println!("Grid height: {}", grid.grid.len());

    let mut steps = 0;
    pos.x += 1;
    while grid.get(pos.x, pos.y) != 'S' {
        pos.step(&grid);
        steps += 1;
        //pos.print(&grid);
        //thread::sleep(time::Duration::from_millis(1));
    }
    println!("steps: {}", steps / 2 + 1);
}
