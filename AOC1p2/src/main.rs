use std::collections::HashMap;
use std::fs::File;
//use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input";
    let keywords = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    //let path = Path::new("output");
    let mut results: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut digits = Vec::new();

            for i in 0..line.len() {
                // Sprawdź czy jest cyfra
                if line[i..].chars().next().unwrap().is_numeric() {
                    digits.push(line[i..].chars().next().unwrap().to_string());
                }
                for (word, digit) in &keywords {
                    if line[i..].starts_with(word) {
                        digits.push(digit.to_string());
                    }
                }
            }

            println!("{}", line);
            //println!("{}", line);
            let mut num_vec: Vec<char> = Vec::new();
            for char in line.chars() {
                if char.is_numeric() {
                    num_vec.push(char);
                } else {
                    continue;
                }
            }
            //println!("{:?}", num_vec);
            if !digits.is_empty() {
                let first = &digits[0];
                let last = &digits[digits.len() - 1];
                let combined = format!("{}{}", first, last);
                let result = combined.parse::<i32>().unwrap();
                results.push(result);
            }
        }
        let final_sum: i32 = results.iter().sum();
        println!("{:?}", &results);
        println!("{}", final_sum);
        //let result_string = results.join(", ");

        //if let Ok(mut output_file) = File::create(path) {
        //    output_file
        //        .write_all(result_string.as_bytes())
        //        .expect("err");
        //} else {
        //    println!("err creating");
        //}
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
