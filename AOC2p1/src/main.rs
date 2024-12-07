use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let max_cubes = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);
    let filename = "input";

    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;

        for line in lines {
            if let Ok(line_content) = line {
                let game_id = get_game_id(&line_content);
                let game_sets = get_game_sets(&line_content);

                println!("Game {}: {:?}", game_id, game_sets);
                let is_game_possible = game_sets.iter().all(|set| {
                    set.iter()
                        .all(|(color, &count)| match max_cubes.get(color) {
                            Some(&max_count) => count <= max_count,
                            None => false,
                        })
                });
                if is_game_possible {
                    sum += game_id;
                }
                println!(
                    "Game {} is {}",
                    game_id,
                    if is_game_possible {
                        "possible"
                    } else {
                        "impossible"
                    }
                );
            }
        }
        println!("Sum of possible game IDs: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn get_game_id(line: &str) -> i32 {
    let game_part = line.split(':').next().unwrap();
    game_part
        .trim_start_matches("Game ")
        .parse::<i32>()
        .unwrap()
}

fn get_game_sets(line: &str) -> Vec<HashMap<String, i32>> {
    let sets_part = line.split(':').nth(1).unwrap();
    sets_part
        .split(';')
        .map(|set| {
            let mut cube_counts = HashMap::new();

            for cube in set.split(',') {
                let parts: Vec<&str> = cube.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    let count = parts[0].parse::<i32>().unwrap();
                    let color = parts[1].trim_end_matches(',').to_string();
                    cube_counts.insert(color, count);
                }
            }
            cube_counts
        })
        .collect()
}
