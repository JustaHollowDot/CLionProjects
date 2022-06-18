use std::cmp::{max, min};
use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        println!("{height:?}");

        let mut a = 0;
        let mut b = height.len() - 1;
        let mut volume= 0;

        loop {


            if a == b {
                break;
            }

            let height_1 = height[a];
            let height_2 = height[b];

            let distance = b - a;

            volume = max(min(height_1, height_2)* distance as i32, volume);

            println!("{volume}");

            if height_1 < height_2 {
                a += 1;
            } else {
                b -= 1;
            }

            println!("{a} {b} : {height_1} {height_2}");
        }

        return volume;
    }
}