use std::fs;
use scan_rules::readln;
use crate::Solution;


impl Solution {
    pub fn find_gamma_and_epsilon () {

        let contents = fs::read_to_string("input").unwrap();

        println!("{contents:?}");

        let mut vector: Vec<Vec<u32>> = vec![vec![]; contents.split_once("\r\n").unwrap().0.len()];

        println!("{:?}", contents.split_once("\r\n").unwrap().0.len());

        for (i, content) in contents.split("\r\n").enumerate() {

            for (i, char) in content.chars().enumerate() {

                vector[i].push(char.to_digit(2).unwrap());
            }
        }


        let mut kake = vec![];
        for list in vector {
            let mut one = 0;
            let mut zero = 0;

            for number in list {

                if number == 1 {
                    one += 1;
                } else {
                    zero += 1;
                }
            }

            if one > zero {
                kake.push(1);
            } else {
                kake.push(0);
            }
        }

        println!("most common bits: {:?}", kake);

        let mut int = 0;
        for i in 0..kake.len() {
            int += kake[kake.len() - i - 1] * 2_i32.pow(i as u32);
        }

        println!("{}", int);


        let mut inverted_kake = vec![];

        for number in kake {
            if number == 0 {
                inverted_kake.push(1);
            } else {
                inverted_kake.push(0);
            }
        }

        println!("least common bits: {:?}", inverted_kake);

        let mut int2 = 0;
        for i in 0..inverted_kake.len() {
            int2 += inverted_kake[inverted_kake.len() - i - 1] * 2_i32.pow(i as u32);
        }

        println!("{}", int2);

        println!("{}", int2 * int)

    }

    pub fn find_most_common_sequence () {

        let contents = fs::read_to_string("input").unwrap();

        let temp_vector : Vec<&str> = contents.split("\r\n").collect();
        let mut contents : Vec<Vec<i32>> = vec![vec![]; temp_vector.len()];

        for (i, vector) in temp_vector.iter().enumerate() {
            for char in vector.chars() {
                contents[i].push(char.to_digit(2).unwrap() as i32)
            }
        }

        println!("contents: {contents:?}");

        let mut contents2 = contents.clone();

        let mut counter : usize = 0;
        while contents.len() > 1  {
            println!("\n\n");

            let contents_columns = Solution::create_contents_columns(&contents);
            println!("columns: {contents_columns:?}");

            let most_common_bit_vec = Solution::find_common_bits(contents_columns);
            println!("common bits: {:?}", most_common_bit_vec);

            contents = Solution::get_all_common_sequences(contents, most_common_bit_vec, counter);
            println!("contents: {contents:?}");



            counter += 1;
        }

        println!("\n\n\n done \n\n\n");




        counter = 0;
        while contents2.len() > 1  {
            println!("\n\n");

            let contents_columns = Solution::create_contents_columns(&contents2);
            println!("columns: {contents_columns:?}");

            let most_common_bit_vec = Solution::find_common_bits(contents_columns);
            println!("common bits: {:?}", most_common_bit_vec);

            contents2 = Solution::get_all_common_sequences_inverted(contents2, most_common_bit_vec, counter);
            println!("contents2: {contents:?}");



            counter += 1;
        }

        println!("\n\n\n done \n\n\n");

        println!("{contents:?}");
        println!("{contents2:?}");

        let len = contents[0].len();
        println!("len: {len}");
        let len2 = contents2[0].len();

        let mut int = 0;
        for i in 0..len {
            int += contents[0][len - i - 1] * 2_i32.pow(i as u32);
        }

        let mut int2 = 0;
        for i in 0..len2 {
            int2 += contents2[0][len2 - i - 1] * 2_i32.pow(i as u32);
        }

        println!("{} : {}", int2, int);

        println!("{}", int2 * int)


    }

    fn get_all_common_sequences (contents : Vec<Vec<i32>>, common_bits : Vec<i32>, counter : usize) -> Vec<Vec<i32>>{
        contents.into_iter().filter(|x| x[counter] == common_bits[counter]).collect::<Vec<Vec<i32>>>()
    }

    fn get_all_common_sequences_inverted (contents : Vec<Vec<i32>>, common_bits : Vec<i32>, counter : usize) -> Vec<Vec<i32>>{
        contents.into_iter().filter(|x| x[counter] != common_bits[counter]).collect::<Vec<Vec<i32>>>()
    }

    fn create_contents_columns (contents : &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut contents_columns: Vec<Vec<i32>> = vec![vec![]; contents[0].len()];

        for content in contents {
            for (i, number) in content.iter().enumerate() {
                contents_columns[i].push(*number);
            }
        }

        contents_columns
    }

    fn find_common_bits (contents_columns : Vec<Vec<i32>>) -> Vec<i32>{
        let mut most_common_bit_vec = vec![];
        for content_column in contents_columns {
            let mut one = 0;
            let mut zero = 0;

            for number in content_column {
                if number == 1 {
                    one += 1;
                } else {
                    zero += 1;
                }
            }

            if one >= zero {
                most_common_bit_vec.push(1);
            } else {
                most_common_bit_vec.push(0);
            }
        }

        most_common_bit_vec
    }
}