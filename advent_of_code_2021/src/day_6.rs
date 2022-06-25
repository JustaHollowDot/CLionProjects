use std::fs::read_to_string;
use crate::Solution;

impl Solution {
    pub fn fish_80 () {
        let contents = read_to_string("input").unwrap();
        let contents : Vec<&str> = contents.split(",").collect();
        let mut start_fish : Vec<usize> = vec![];

        for content in contents {
            start_fish.push(content.parse().unwrap())
        }

        let mut fish_per_day = vec![0; 9];

        for fish in start_fish {
            fish_per_day[fish] += 1;
        }

        for i in 0..=80 {
            fish_per_day.rotate_left(1);

            fish_per_day[5] += fish_per_day[7];
        }

        println!("{}", fish_per_day.iter().sum::<i64>());
    }

    pub fn fish_256 () {
        let contents = read_to_string("input").unwrap();
        let contents : Vec<&str> = contents.split(",").collect();
        let mut start_fish : Vec<usize> = vec![];

        for content in contents {
            start_fish.push(content.parse().unwrap())
        }

        let mut fish_per_day = vec![0; 9];

        for fish in start_fish {
            fish_per_day[fish] += 1;
        }

        for i in 0..=256 {
            fish_per_day.rotate_left(1);

            fish_per_day[5] += fish_per_day[7];
        }

        println!("{}", fish_per_day.iter().sum::<i64>());
    }
}