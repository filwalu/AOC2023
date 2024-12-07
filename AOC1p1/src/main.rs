use std::fs::File;
//use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input";
    //let path = Path::new("output");
    let mut results: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
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
            if !num_vec.is_empty() {
                let first = num_vec[0];
                let last: char = num_vec[num_vec.len() - 1];
                //println!("{:?}", first);
                //println!("{:?}", last);
                let first_digit = first.to_string();
                let last_digit = last.to_string();
                let combined = format!("{}{}", first_digit, last_digit);
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
