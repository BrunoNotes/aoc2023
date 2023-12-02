use std::{env, fs, i32};

fn main() {
    let file_path = format!(
        "{}/part1/src/input.txt",
        env::current_dir().unwrap().display()
    );
    let input = fs::read_to_string(file_path).unwrap();

    let mut result: i32 = 0;
    for line in input.lines() {
        // let test = line.chars().skip_while(|c| c.is_digit(10)).map(|c| c);
        let mut numbers_from_line: Vec<char> = Vec::new();
        for c in line.chars() {
            if c.is_digit(10) {
                // println!("{:?}", c);
                numbers_from_line.push(c.into());
            }
        }
        // println!("{:?}", numbers_from_line);
        let first = numbers_from_line.first().unwrap();
        let last = numbers_from_line.last().unwrap();

        let calibration: i32 = format!("{}{}", first, last).parse::<i32>().unwrap();
        // println!("{}", calibration);
        result += calibration;
    }
    println!("{:?}", result);
}
