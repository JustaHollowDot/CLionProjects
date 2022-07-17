use itertools::Itertools;
use crate::{get_input, Solution};

fn get_hexadecimal_input_as_bit_string() -> String {
    let contents = get_input();

    let mut contents_number = Vec::new();

    for char in contents.chars() {
        contents_number.push(char.to_digit(16).unwrap())
    }

    println!("contents: {}", contents);



    let mut byte_string = String::new();

    let mut new_string = String::new();

    for number in contents_number {
        for i in 0..4 {
            byte_string.push_str(&((number>>i)&1).to_string())
        }

        byte_string = byte_string.chars().rev().collect::<String>();

        new_string.push_str(&byte_string);

        byte_string.clear();
    }

    new_string
}

fn decode_package(new_string: String, version_counter: &mut i32) {
    println!("new_string: {}", new_string);

    let test = new_string.chars().take(3).collect::<String>();


    let packet_version = {
        let mut number = 0;
        test.chars().rev().enumerate().for_each(|(i, char)| {
            number |= char.to_digit(2).unwrap() << i;
        });

        number
    };

    println!("packet_version: {}", packet_version);
    *version_counter += packet_version as i32;

    let test2 = new_string.chars().skip(3).take(3).collect::<String>();
    let packet_id = {
        let mut number = 0;
        test2.chars().rev().enumerate().for_each(|(i, char)| {
            number |= char.to_digit(2).unwrap() << i;
        });

        number
    };

    println!("packet_id: {}", packet_id);

    let packages = new_string.chars().skip(6).collect::<String>();

    println!("packages: {}", packages);



    if packet_id == 4 {
        let mut number: Vec<char> = Vec::new();

        let mut packages_iter = packages.chars();

        loop {
            let first = packages_iter.next().unwrap();

            println!("first: {:?}", first);

            let number_vec = packages_iter.clone().take(4).collect::<Vec<char>>();

            for number_part in &number_vec {
                number.push(*number_part)
            }

            println!("1-4: {:?}", number_vec);

            for _ in 0..4 {
                packages_iter.next();
            }

            if first == '0' { break }

        }

        println!("number: {:?}", number);

        println!("len: {}", number.len());

        let final_number = {
            let mut counter: u128 = 0;
            number.iter().rev().enumerate().for_each(|(i, char)| {
                counter |= ((char.to_digit(2).unwrap() as u128) << i) as u128;
            });

            counter
        };

        println!("number: {}", final_number);

        println!("remaining package: {:?}", packages_iter);

        if packages_iter.clone().contains(&'1') {
            decode_package(packages_iter.collect::<String>(), version_counter);
        }


    } else {
        let mut packages_iter = packages.chars();
        let id = packages_iter.next().unwrap();

        println!("id: {}", id);

        if id == '0' {
            let next_amount_iter = packages_iter.clone().take(15).collect::<Vec<char>>();

            for _ in 0..15 {
                packages_iter.next();
            }

            let next_amount = {
                let mut counter = 0;
                next_amount_iter.iter().rev().enumerate().for_each(|(i, char)| {
                    counter |= char.to_digit(2).unwrap() << i;
                });

                counter
            };

            println!("next_amount: {}", next_amount);

            println!("{:?}", packages_iter);


            decode_package(packages_iter.collect::<String>(), version_counter);

        } else {
            let next_amount_iter = packages_iter.clone().take(11).collect::<Vec<char>>();

            for _ in 0..11 {
                packages_iter.next();
            }

            let next_amount = {
                let mut counter = 0;
                next_amount_iter.iter().rev().enumerate().for_each(|(i, char)| {
                    counter |= char.to_digit(2).unwrap() << i;
                });

                counter
            };

            println!("next_count: {}", next_amount);

            println!("{:?}", packages_iter);


            decode_package(packages_iter.collect::<String>(), version_counter);
        }
    }



}

impl Solution {
    pub fn packet_decoder() {
        let new_string = get_hexadecimal_input_as_bit_string();

        let mut version_counter = 0;
        decode_package(new_string, &mut version_counter);

        println!("version_counter: {}", version_counter);
    }

    pub fn calculate_from_packages() {

    }
}