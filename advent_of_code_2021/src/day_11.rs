use std::ops::Index;
use std::str::Chars;
use itertools::Itertools;
use crate::{create_vec_vec_uint, get_input, Solution};



impl Solution {
    pub fn octopuses () {
        let input = get_input();

        let mut vec = create_vec_vec_uint(input);

        let len = vec.len();

        let mut binary_vec  = vec![vec![false; len]; len];

        for line in &vec {
            println!("{:?}", line);
        }

        let mut sum = 0;

        for i in 0..100 {
            while vec.iter().flatten().collect::<Vec<&u32>>().contains(&&9) {
                let mut vector = vec.iter().flatten().collect::<Vec<&u32>>();
                let idx = vector.iter().position(|x| **x == 9).unwrap();



                let i = idx / vec.len();
                let i2 = idx % vec.len();

                Solution::increment_adjacent(&mut vec, &mut binary_vec, i, i2, &mut sum);

                vec[i][i2] += 1;

            }


            for line in vec.iter_mut() {
                for number in line.iter_mut() {
                    *number += 1;

                    if *number > 9 {
                        *number = 0;

                    }
                }
            }

            let test = binary_vec.iter().flatten().all(|x| *x == true);
            if test {
                println!("{}", i);
                break;
            }

            binary_vec  = vec![vec![false; len]; len];
        }

        println!(" ");

        for line in &vec {
            println!("{:?}", line);
        }

        println!("{}", sum);
    }

    fn increment_adjacent (mut vec: &mut Vec<Vec<u32>>, binary_vec: &mut Vec<Vec<bool>>, i: usize, i2: usize, sum: &mut i32) {
        *sum += 1;
        binary_vec[i][i2] = true;

        if i2 > 0 {

            vec[i][i2 - 1] += 1;

            if vec[i][i2 - 1] > 9 && !binary_vec[i][i2 - 1] {
                Solution::increment_adjacent(vec, binary_vec, i, i2 - 1, sum);
            }
        }
        if i2 < vec.len() - 1 {

            vec[i][i2 + 1] += 1;

            if vec[i][i2 + 1] > 9 && !binary_vec[i][i2 + 1] {
                Solution::increment_adjacent(vec, binary_vec, i, i2 + 1, sum);
            }
        }



        if i < vec.len() - 1 {
            if i2 == 0 {
                for j in 0..2 {

                    vec[i + 1][i2 + j] += 1;

                    if vec[i + 1][i2 + j] > 9 && !binary_vec[i + 1][i2 + j] {
                        Solution::increment_adjacent(vec, binary_vec, i + 1, i2 + j, sum);
                    }
                }
            } else if i2 == vec.len() - 1 {
                for j in 0..2 {

                    vec[i + 1][i2 - 1 + j] += 1;

                    if vec[i + 1][i2 - 1 + j] > 9 && !binary_vec[i + 1][i2 - 1 + j] {
                        Solution::increment_adjacent(vec, binary_vec, i + 1, i2 - 1 + j, sum);
                    }
                }
            } else {
                for j in 0..3 {

                    vec[i + 1][i2 - 1 + j] += 1;

                    if vec[i + 1][i2 - 1 + j] > 9 && !binary_vec[i + 1][i2 - 1 + j] {
                        Solution::increment_adjacent(vec, binary_vec, i + 1, i2 - 1 + j, sum);
                    }
                }
            }
        }

        if i > 0 {
            if i2 == 0 {
                for j in 0..2 {

                    vec[i - 1][i2 + j] += 1;

                    if vec[i - 1][i2 + j] > 9 && !binary_vec[i - 1][i2 + j] {
                        Solution::increment_adjacent(vec, binary_vec, i - 1, i2 + j, sum);
                    }
                }
            } else if i2 == vec.len() - 1 {
                for j in 0..2 {

                    vec[i - 1][i2 - 1 + j] += 1;

                    if vec[i - 1][i2 - 1 + j] > 9 && !binary_vec[i - 1][i2 - 1 + j] {
                        Solution::increment_adjacent(vec, binary_vec, i - 1, i2 - 1 + j, sum);
                    }
                }
            } else {
                for j in 0..3 {

                    vec[i - 1][i2 - 1 + j] += 1;


                    if vec[i - 1][i2 - 1 + j] > 9 && !binary_vec[i - 1][i2 - 1 + j] {
                        Solution::increment_adjacent(vec, binary_vec, i - 1, i2 - 1 + j, sum);
                    }
                }
            }
        }
    }
}