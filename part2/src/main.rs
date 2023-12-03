use std::{collections::HashMap, env, fmt::format, fs};

const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[derive(Debug)]
struct ElfGameDesc {
    number: String,
    cubes: String,
}

#[derive(Debug)]
struct Cube {
    color: String,
    quantity: String,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct CubeColorQuantity {
    red: i32,
    green: i32,
    blue: i32,
}

// #[derive(Debug, Hash, Eq, PartialEq)]
// struct Game {
//     game: String,
// }

fn main() {
    let file_path = format!(
        "{}/part2/src/input.txt",
        env::current_dir().unwrap().display()
    );

    let input = fs::read_to_string(file_path).unwrap();
    // let input = TEST_INPUT.to_string();

    let game_cubes: Vec<ElfGameDesc> = input
        .lines()
        .map(|line| {
            let input_split: Vec<&str> = line.split(':').collect();
            ElfGameDesc {
                number: input_split.first().unwrap().to_string(),
                cubes: input_split.last().unwrap().to_string(),
            }
        })
        .collect();

    let game: HashMap<_, _> = game_cubes
        .iter()
        .map(|game| {
            let game_number = game
                .number
                .chars()
                .skip_while(|n| !n.is_digit(10))
                .skip_while(|n| n.to_string() == "".to_string())
                .collect::<String>();

            let cube_color: Vec<_> = game
                .cubes
                .split(';')
                .map(|c| c.split(',').collect::<Vec<_>>())
                .map(|c| {
                    let color_quantity = c
                        .into_iter()
                        .map(|cubes| cubes.split_whitespace().collect::<Vec<_>>())
                        .map(|cubes| {
                            let color: String = cubes
                                .clone()
                                .into_iter()
                                .map(|cube| {
                                    cube.chars()
                                        .skip_while(|n| n.is_digit(10))
                                        .collect::<String>()
                                })
                                .collect::<_>();

                            let quantity: String = cubes
                                .into_iter()
                                .map(|cube| {
                                    cube.chars()
                                        .skip_while(|n| !n.is_digit(10))
                                        .collect::<String>()
                                })
                                .collect::<_>();

                            Cube {
                                color: color,
                                quantity: quantity,
                            }
                        })
                        .collect::<Vec<_>>();
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    for cube in color_quantity {
                        match cube.color.as_str() {
                            "red" => red += cube.quantity.parse::<i32>().unwrap(),
                            "green" => green += cube.quantity.parse::<i32>().unwrap(),
                            "blue" => blue += cube.quantity.parse::<i32>().unwrap(),
                            _ => (),
                        }
                    }

                    CubeColorQuantity {
                        red: red,
                        green: green,
                        blue: blue,
                    }
                })
                .collect();

            // let mut red = 0;
            // let mut green = 0;
            // let mut blue = 0;
            //
            // for color in cube_color {
            //     red += color.red;
            //     green += color.green;
            //     blue += color.blue;
            // }

            (
                game_number,
                // CubeColorQuantity {
                //     red: red,
                //     green: green,
                //     blue: blue,
                // },
                cube_color,
            )
        })
        .collect();

    let max_cube_quantity = CubeColorQuantity {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut total_valid = 0;
    // println!("{:?}", game);
    for (round, cube) in game {
        let mut is_valid: bool= true;

        for quantity in cube {
            if quantity.red > max_cube_quantity.red {
                is_valid = false
            } else if quantity.blue > max_cube_quantity.blue {
                is_valid = false
            } else if quantity.green > max_cube_quantity.green {
                is_valid = false
            } 
        }

        // println!(
        //     "Round: {:?}, Cube: {:?}, Valid: {:?}",
        //     round,  cube, is_valid
        // );

        if is_valid == true {
            println!("{:?}", round);
            total_valid += round.parse::<i32>().unwrap();
        }
    }
    println!("Total valid: {:?}", total_valid);
}
