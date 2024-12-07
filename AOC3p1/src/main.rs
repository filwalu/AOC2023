use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SPECIAL_CHARS: [char; 12] = ['$', '!', '@', '#', '%', '^', '&', '*', '-', '+', '=', '/'];
struct NumberPosition {
    row: usize,
    col: usize,
    value: i32,
}
fn main() {
    let filename = "input";
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_content) = line {
                let row: Vec<char> = line_content.chars().collect();
                grid.push(row);
            }
        }

        let number_positions = get_number_positions(&grid);
        for pos in number_positions {
            if look_for_special_chars(&pos, &grid) {
                sum += pos.value;
                println!(
                    "Found number {} at position ({}, {}) - adjacent to special char, adding to sum. Current sum: {}",
                    pos.value, pos.row, pos.col, sum
                );
            } else {
                println!(
                    "Found number {} at position ({}, {}) - no special chars, skipping",
                    pos.value, pos.row, pos.col
                );
            }
        }

        println!("Final sum: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn get_number_positions(grid: &Vec<Vec<char>>) -> Vec<NumberPosition> {
    let mut positions = Vec::new();
    let mut current_number = String::new();
    let mut start_col = 0;

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

fn look_for_special_chars(number: &NumberPosition, grid: &Vec<Vec<char>>) -> bool {
    let row = number.row as i32;
    let start_col = number.col as i32;
    let end_col = (number.col + number.value.to_string().len() - 1) as i32;

    let is_special_char = |ch: char| SPECIAL_CHARS.contains(&ch);

    for r in (row - 1)..=(row + 1) {
        for c in (start_col - 1)..=(end_col + 1) {
            if r >= 0 && (r as usize) < grid.len() && c >= 0 && (c as usize) < grid[0].len() {
                let char_at_position = grid[r as usize][c as usize];
                if is_special_char(char_at_position) {
                    return true;
                }
            }
        }
    }
    false
}
