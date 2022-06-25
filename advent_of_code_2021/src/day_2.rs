use std::fs;
use text_io::read;
use crate::Solution;

impl Solution {
    pub fn find_final_position () {
        let contents = fs::read_to_string("input").unwrap();

        let mut bool = false;
        let mut test : u8 = 0;

        let mut forward = 0;
        let mut depth = 0;

        for content in contents.split("`\n") {
            for word in content.split_whitespace() {
                println!("{word}");

                if bool == false {
                    match word.chars().next().unwrap() {
                        'f' => test = 1,
                        'd' => test = 2,
                        'u' => test = 3,
                        _ => { }
                    };
                    bool = true;

                } else {
                    let number : i32 = word.trim().parse().expect("invalid");

                    match test {
                        1 => forward += number,
                        2 => depth += number,
                        3 => depth -= number,
                        _ => { println!("invalid") }
                    }

                    bool = false;
                }
            }
        }

        println!("{forward} : {depth} => {}", forward * depth);
    }

    pub fn find_final_position_with_aim () {
        let contents = fs::read_to_string("input").unwrap();

        let mut bool = false;
        let mut test : u8 = 0;

        let mut forward = 0;
        let mut depth = 0;
        let mut aim = 0;

        for content in contents.split("`\n") {
            for word in content.split_whitespace() {
                println!("{word}");

                if bool == false {
                    match word.chars().next().unwrap() {
                        'f' => test = 1,
                        'd' => test = 2,
                        'u' => test = 3,
                        _ => { }
                    };
                    bool = true;

                } else {
                    let number : i32 = word.trim().parse().expect("invalid");

                    match test {
                        1 => {
                            forward += number;
                            depth += (number * aim)
                        },
                        2 => aim += number,
                        3 => aim -= number,
                        _ => { println!("invalid") }
                    }

                    bool = false;
                }
            }
        }

        println!("{forward} : {depth} => {}", forward * depth);
    }
}