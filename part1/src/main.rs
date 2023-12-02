use std::{env, fs};

fn main() {
    let file_path = format!(
        "{}/part1/src/input.txt",
        env::current_dir().unwrap().display()
    );
    // let file = fs::File::open(file_path).unwrap();
    // let mut buf_reader = BufReader::new(file);
    // let mut input = String::new();
    // buf_reader.read_to_string(&mut input).unwrap();

    let input = fs::read_to_string(file_path).unwrap();

    //     let input = "two1nine
    // eightwothree
    // abcone2threexyz
    // xtwone3four
    // 4nineeightseven2
    // zoneight234
    // 7pqrstsixteen
    // ".to_string();

    // let mut n_line = 1;

    let output: u32 = input
        .lines()
        .map(|line| {
            let line_replace = (0..line.len()).filter_map(|index| {
                let line_by_index = &line[index..];

                let result = if line_by_index.starts_with("one") {
                    Some('1')
                } else if line_by_index.starts_with("two") {
                    Some('2')
                } else if line_by_index.starts_with("three") {
                    Some('3')
                } else if line_by_index.starts_with("four") {
                    Some('4')
                } else if line_by_index.starts_with("five") {
                    Some('5')
                } else if line_by_index.starts_with("six") {
                    Some('6')
                } else if line_by_index.starts_with("seven") {
                    Some('7')
                } else if line_by_index.starts_with("eight") {
                    Some('8')
                } else if line_by_index.starts_with("nine") {
                    Some('9')
                } else {
                    line_by_index.chars().next()
                };
                result
            });

            let mut x = line_replace.filter_map(|c| c.to_digit(10));

            let first = x.next().unwrap();
            // let last = x.last().unwrap();

            match x.last() {
                Some(n) => format!("{first}{n}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .unwrap()
        })
        .sum();

    // let output = input.lines().map(process_line).sum::<u32>();
    println!("{output}");
}
