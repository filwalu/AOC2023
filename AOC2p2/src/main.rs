use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input";

    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;

        for line in lines {
            if let Ok(line_content) = line {
                let game_sets = get_game_sets(&line_content);
                let max_colors = get_max_color_count(&game_sets);
                //println!("{:?},{:?}", game_sets, max_colors);
                println!("{:?}", max_colors);
                let power = get_power(&max_colors);
                println!("{}", power);
                sum += power;
                println!("{}", sum);
            }
        }
        println!("{}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn get_max_color_count(game_sets: &Vec<HashMap<String, i32>>) -> Vec<HashMap<String, i32>> {
    let mut max_colors: HashMap<String, i32> = HashMap::new();
    for set in game_sets {
        for (color, &count) in set {
            max_colors
                .entry(color.clone())
                .and_modify(|e| *e = (*e).max(count))
                .or_insert(count);
        }
    }
    vec![max_colors]
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
fn get_power(max_colors: &Vec<HashMap<String, i32>>) -> i32 {
    if let Some(colors) = max_colors.first() {
        let red = colors.get("red").unwrap_or(&0);
        let green = colors.get("green").unwrap_or(&0);
        let blue = colors.get("blue").unwrap_or(&0);

        red * green * blue
    } else {
        0
    }
}
