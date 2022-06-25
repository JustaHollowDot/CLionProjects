use std::io::Read;
use std::fs;
use crate::Solution;

impl Solution {
    pub fn find_amount_increasing () -> i32 {

        let mut file = std::fs::File::open("input").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut previous = 0;
        let mut counter = -1;
        for content in contents.split_whitespace() {
            let current : i32 = content.trim().parse().expect("invalid input");

            if current > previous {
                counter += 1;
            }

            previous = current;
        }


        println!("{counter}");
        return 0;

    }
}

impl Solution {
    pub fn find_amount_of_average_increasing () -> i32 {

        let mut file = std::fs::File::open("input").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut vector = vec![0; 2002];

        for (i, content) in contents.split_whitespace().enumerate() {
            let current : i32 = content.trim().parse().expect("invalid");

            vector[i] += current;
            vector[i + 1] += current;
            vector[i + 2] += current;

        }

        let mut counter = 0;
        for i in 2..vector.len() - 2 {
            println!("{}, {}", i, vector[i]);

            if vector[i + 1] > vector[i] {
                counter += 1;
            }
        }

        println!("{}", counter);

        return 0;
    }
}