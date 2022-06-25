use std::fs::read_to_string;
use itertools::Itertools;
use crate::Solution;

impl Solution {
    pub fn display () {
        let contents = read_to_string("input").unwrap();

        let contents : Vec<&str> = contents.lines().collect();

        let mut temp = vec![];
        for content in contents {
            let split_content : Vec<&str> = content.split("|").collect();

            temp.push(split_content);
        }

        let mut contents = vec![];

        for (i, content) in temp.iter().enumerate() {
            contents.push(vec![]);

            for (i2, string) in content.iter().enumerate() {

                let words : Vec<&str> = string.split_whitespace().collect();

                contents[i].push(words)
            }
        }

        for content in &contents {
            println!("{:?}", content);
        }

        let mut amount = 0;
        for content in &contents {
            for words in &content[1] {
                match words.len() {
                    2 => amount += 1,
                    3 => amount += 1,
                    4 => amount += 1,
                    7 => amount += 1,
                    _ => {}
                }
            }
        }

        println!("{}", amount);
    }

    pub fn display_with_numbers () {
        let contents = read_to_string("input").unwrap();

        let contents : Vec<&str> = contents.lines().collect();

        let mut temp = vec![];
        for content in contents {
            let split_content : Vec<&str> = content.split("|").collect();

            temp.push(split_content);
        }

        let mut contents = vec![];

        for (i, content) in temp.iter().enumerate() {
            contents.push(vec![]);

            for (i2, string) in content.iter().enumerate() {

                let words : Vec<&str> = string.split_whitespace().collect();

                contents[i].push(words)
            }
        }

        for content in &contents {
            println!("{:?}", content);
        }



        let mut sum = 0;

        for content in &contents {
            let mut vec_numbers = vec![" "; 10];
            let mut unknown_6 = vec![];
            let mut unknown_5 = vec![];

            println!("{:?}", content[0]);

            for word in &content[0] {

                if word.len() == 2 {
                    vec_numbers[1] = word;
                } else if word.len() == 3 {
                    vec_numbers[7] = word;
                } else if word.len() == 4 {
                    vec_numbers[4] = word;
                } else if word.len() == 7 {
                    vec_numbers[8] = word;
                } else if word.len() == 6 {
                    unknown_6.push(word);
                } else {
                    unknown_5.push(word);
                }


            }


            'outer: for word in unknown_6 {
                for char in vec_numbers[1].chars() {
                    if !word.contains(char) {
                        vec_numbers[6] = word;
                        continue 'outer
                    }
                }

                for char in vec_numbers[4].chars() {
                    if !word.contains(char) {
                        vec_numbers[0] = word;
                        continue 'outer
                    }
                }

                vec_numbers[9] = word;
            }

            'outer: for word in unknown_5 {
                for char in vec_numbers[1].chars() {
                    if !word.contains(char) {
                        for char_2 in vec_numbers[4].chars() {
                            if char == char_2 {
                                continue
                            }

                            if !word.contains(char_2) {
                                vec_numbers[2] = word;
                                continue 'outer
                            }

                        }

                        vec_numbers[5] = word;
                        continue 'outer
                    }
                }

                vec_numbers[3] = word
            }

            println!("{:#?}", vec_numbers);

            let mut temp_sum = 0;

            for (i, word) in content[1].iter().enumerate() {
                let index = vec_numbers.iter().position(|&r| r.chars().sorted().collect::<String>() == word.chars().sorted().collect::<String>()).unwrap();

                println!("{}", index);

                temp_sum += index * (10_i32.pow((3 - i).try_into().unwrap()) as usize );
            }

            println!("{}", temp_sum);
            sum += temp_sum;


        }

        println!("{}", sum);
    }
}