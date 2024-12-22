use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


const SPECIAL_CHAR: char = '*';
#[derive(Hash, Eq, PartialEq)]
struct StarPosition {
    row: usize,
    col: usize,
}
struct NumberPosition {
    row: usize,
    col: usize,
    value: i32,
}
fn main() {
    let filename: &str = "input";
    let mut grid: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_content) = line {
                let row: Vec<char> = line_content.chars().collect();
                grid.push(row);
            }
        }

        let number_positions: Vec<NumberPosition> = get_number_positions(&grid);
        let mut star_numbers: HashMap<StarPosition, Vec<u32>> = HashMap::new();

        for pos in number_positions {
            if let Some(star_pos) = look_for_special_chars(&pos, &grid) {
                star_numbers.entry(star_pos)
                    .or_insert(Vec::new())
                    .push(pos.value.try_into().unwrap());
            }
        }
        
        let gear_ratio_sum: u32 = star_numbers
            .values()
            .filter(|numbers| numbers.len() == 2)
            .map(|numbers| numbers[0] * numbers[1])
            .sum();

        println!("Final sum: {}", gear_ratio_sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn get_number_positions(grid: &Vec<Vec<char>>) -> Vec<NumberPosition> {
    let mut positions: Vec<NumberPosition> = Vec::new();
    let mut current_number: String = String::new();
    let mut start_col: usize = 0;

    for (row, line) in grid.iter().enumerate() {
        current_number.clear();

        for (col, &ch) in line.iter().enumerate() {
            if ch.is_ascii_digit() {
                if current_number.is_empty() {
                    start_col = col;
                }
                current_number.push(ch);
            } else if !current_number.is_empty() {
                if let Ok(value) = current_number.parse::<i32>() {
                    positions.push(NumberPosition {
                        row,
                        col: start_col,
                        value,
                    });
                }
                current_number.clear();
            }
        }

        if !current_number.is_empty() {
            if let Ok(value) = current_number.parse::<i32>() {
                positions.push(NumberPosition {
                    row,
                    col: start_col,
                    value,
                });
            }
        }
    }

    positions
}

fn look_for_special_chars(number: &NumberPosition, grid: &Vec<Vec<char>>) -> Option<StarPosition> {
    let row = number.row as i32;
    let start_col = number.col as i32;
    let end_col = (number.col + number.value.to_string().len() - 1) as i32;

    for r in (row - 1)..=(row + 1) {
        for c in (start_col - 1)..=(end_col + 1) {
            if r >= 0 && (r as usize) < grid.len() && c >= 0 && (c as usize) < grid[0].len() {
                if grid[r as usize][c as usize] == SPECIAL_CHAR {
                    return Some(StarPosition {
                        row: r as usize,
                        col: c as usize,
                    });
                }
            }
        }
    }
    None
}
//REVERSE LOGIC SEARCH FOR NUMBER ADJACENT TO * THEN MULTIPLY THESE NUMBERS and add to sum
