use std::cmp::{max, min};
use std::fs::read_to_string;
use crate::Solution;

impl Solution {
    pub fn hydrothermal_venture() {
        let contents = read_to_string("input").unwrap();

        let contents: Vec<&str> = contents.lines().collect();

        let mut positions: Vec<Vec<Vec<i32>>> = vec![];
        for (i, content) in contents.iter().enumerate() {
            positions.push(vec![]);

            let vector_string = content.split(" -> ").collect::<Vec<&str>>();

            for (i2, x_y_string) in vector_string.iter().enumerate() {
                positions[i].push(vec![]);

                let x_y_vector: Vec<&str> = x_y_string.split(",").collect();

                for number in &x_y_vector {
                    let test = number.parse().unwrap();
                    positions[i][i2].push(test)
                }
            }
        }

        for position in &positions {
            println!("{:?}", position);
        }

        let max_number = positions.iter().max().unwrap().iter().max().unwrap().iter().max().unwrap() + 1;

        let mut grid = vec![vec![0; max_number as usize]; max_number as usize];

        for position in &positions {
            if position[0][0] == position[1][0] {
                println!("position: {:?}", position);

                let index = position[0][0];

                let start = min(position[0][1], position[1][1]);
                let end = max(position[0][1], position[1][1]);

                for i in start..=end {
                    grid[i as usize][index as usize] += 1;
                }
            } else if position[0][1] == position[1][1] {
                println!("position: {:?}", position);

                let index = position[0][1];

                let start = min(position[0][0], position[1][0]);
                let end = max(position[0][0], position[1][0]);

                for i in start..=end {
                    grid[index as usize][i as usize] += 1;
                }
            }
        }

        let mut counter = 0;
        for row in &grid {
            for number in row {
                print!("{number} ");
                if number > &1 {
                    counter += 1;
                }
            }
            println!(" ");
        }

        println!("{}", counter);

        for position in &positions {
            if position[0][0] == position[1][0] {

            } else if position[0][1] == position[1][1] {

            } else {
                if position[0][0] - position[1][0] == position[0][1] - position[1][1] {

                    println!("position: {:?}", position);

                    println!("test: {}, {}", position[0][0] - position[1][0], position[0][1] - position[1][1]);

                    let start = min(position[0][0], position[1][0]);
                    let end = max(position[0][0], position[1][0]);

                    println!("start-end-1: {} {}", start, end);

                    for i in start..=end {
                        grid[i as usize][i as usize] += 1;
                    }
                } else if position[0][0] == position[0][1] || position[1][0] == position[1][1] {

                } else {
                    println!("position: {:?}", position);

                    let start = min(position[0][0], position[1][0]);
                    let end = max(position[0][0], position[1][0]);

                    println!("start-end-2: {} {}", start, end);

                    for i in start..=end {
                        grid[i as usize][(end - i) as usize] += 1;
                    }
                }
            }
        }

        let mut counter = 0;
        for row in &grid {
            for number in row {
                print!("{number} ");
                if number > &1 {
                    counter += 1;
                }
            }
            println!(" ");
        }

        println!("{}", counter);
    }
}