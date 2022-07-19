#[allow(unused_imports)]

extern crate core;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;

struct Solution {
}

use std::io::Read;

pub fn get_input () -> String {
    let mut input = std::io::stdin();
    let mut string = String::new();
    input.read_to_string(&mut string).unwrap();

    string
}

pub fn create_vec_vec_uint (input: String) -> Vec<Vec<u32>> {
    let mut lines = input;

    let string_vec = lines.lines().collect::<Vec<&str>>();
    let int_vec_vec = string_vec.iter().map(|s| {
        s.chars().map(|x| {
            x.to_digit(10).unwrap()
        }).collect()
    }).collect::<Vec<Vec<u32>>>();

    int_vec_vec
}

fn main() {

    Solution::calculate_from_packages();


}
