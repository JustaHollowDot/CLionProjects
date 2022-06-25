use std::fs::read_to_string;
use crate::Solution;

impl Solution {
    pub fn crabs () {
        let contents = read_to_string("input").unwrap();

        println!("{}", contents);

        let contents : Vec<&str> = contents.split(",").collect();

        println!("{:?}", contents);

        let mut crabs_position = vec![];
        for content in contents {
            let position : i32 = content.parse().unwrap();

            crabs_position.push(position);
        }

        let mut vector = vec![];
        for i in 0..crabs_position.len() {

            let mut sum = 0;

            for position in &crabs_position {
                sum += (i as i32 - position).abs()
            }

            println!("{}", sum);

            vector.push(sum);
        }

        println!("{}", vector.iter().min().unwrap());
    }

    pub fn crabs_but_costlier () {
        let contents = read_to_string("input").unwrap();

        println!("{}", contents);

        let contents : Vec<&str> = contents.split(",").collect();

        println!("{:?}", contents);

        let mut crabs_position = vec![];
        for content in contents {
            let position : i32 = content.parse().unwrap();

            crabs_position.push(position);
        }

        let mut vector = vec![];
        for i in 0..crabs_position.len() {

            let mut sum = 0;

            for position in &crabs_position {
                sum += ((i as i32 - position).abs() * ((i as i32 - position).abs() + 1)) / 2
            }

            println!("{}", sum);

            vector.push(sum);
        }

        println!("{}", vector.iter().min().unwrap());
    }
}