use std::collections::HashMap;
use std::fs::read_to_string;
use itertools::Itertools;
use crate::{get_input, Solution};

impl Solution {
    pub fn polymerization () {
        let contents = get_input();

        let (line, template) = contents.split_once("\r\n\r\n").unwrap();

        let mut line = line.to_string();

        println!("line: {}\n", line);

        let mut template: Vec<&str> = template.lines().collect();
        template.sort();

        let kake : Vec<Vec<_>>= template.iter().map(|x| {
            x.split(" -> ").collect()
        }).collect();

        println!("{:#?}", kake);

        for i in 0..40 {
            let mut test = String::new();

            let line_iter = line.chars().enumerate();

            line_iter.for_each(|(i,c)| {
                test.push(c);

                let x = line.chars().nth(i + 1);

                if x != None {
                    let index_test = kake.iter().position(|v| {
                        v[0].chars().nth(0).unwrap() == c &&  v[0].chars().nth(1).unwrap() == x.unwrap()
                    });

                    if index_test != None {
                        test.push(kake[index_test.unwrap()][1].chars().nth(0).unwrap());
                    }
                }
            });

            line = test;

            println!("{i}");
        }

        println!("line: {}", line);

        let line_sorted : String = line.chars().sorted().collect();

        println!("{}", line_sorted);

        let mut last = ' ';

        let mut sum = Vec::new();
        let mut counter = 0;
        for char in line_sorted.chars() {
            counter += 1;

            if char != last {
                sum.push(counter);
                counter = 0;
            }

            last = char;
        }

        sum.remove(0);

        println!("{:?}", sum);

        println!("{} : {} -> {}", sum.iter().min().unwrap(), sum.iter().max().unwrap(), sum.iter().max().unwrap() - sum.iter().min().unwrap());
    }

    pub fn polymerization_2 () {
        let contents = get_input();

        let (line, template) = contents.split_once("\r\n\r\n").unwrap();
        let mut line = line.to_string();

        let mut template: Vec<&str> = template.lines().collect();
        template.sort();

        let kake : Vec<Vec<_>>= template.iter().map(|x| {
            x.split(" -> ").collect()
        }).collect();

        let mut vec_test:Vec<_> = Vec::new();

        for vec in &kake {
            vec_test.push(vec![vec![(vec[0].chars().nth(0).unwrap(), ' ')]])
        }

        vec_test.dedup();

        let mut last = ' ';
        let mut counter = 0;
        let mut vec_char:Vec<(char, char)> = Vec::new();

        for vec in kake {
            if vec[0].chars().nth(0).unwrap() != last && last != ' ' {
                vec_test[counter].push(vec_char.clone());
                vec_char.clear();

                counter += 1;
                last = vec[0].chars().nth(0).unwrap();
            } else if vec[0].chars().nth(0).unwrap() != last && last == ' ' {
                last = vec[0].chars().nth(0).unwrap();
            }

            vec_char.push((vec[0].chars().nth(1).unwrap(), vec[1].chars().nth(0).unwrap()));
        }
        vec_test[counter].push(vec_char.clone());

        let mut hash_map: HashMap<char, Vec<(char, char)>> = HashMap::new();

        for vec_2d in &vec_test {
            hash_map.insert(
                vec_2d[0][0].0.clone(),
                vec_2d[1].clone(),
            );
        }

        println!("{:?}", hash_map);


        for i in 0..40 {

            let mut new_line = String::new();
            let mut line_iter = line.chars();
            let mut line_iter_2 = line.chars().skip(1);

            for char_2 in line_iter_2 {
                let char_1 = line_iter.next().unwrap();

                let char_3 = hash_map[&char_1].iter().find(|x| x.0 == char_2).unwrap().1;


                new_line.push(char_1);
                new_line.push(char_3);
            }
            new_line.push(line_iter.next().unwrap());

            println!("{}", i);

            line = new_line;
        }

        let line_sorted : String = line.chars().sorted().collect();

        println!("{}", line_sorted);

        let mut last = ' ';

        let mut sum = Vec::new();
        let mut counter = 0;
        for char in line_sorted.chars() {
            counter += 1;

            if char != last {
                sum.push(counter);
                counter = 0;
            }

            last = char;
        }

        sum.remove(0);

        println!("{:?}", sum);

        println!("{} : {} -> {}", sum.iter().min().unwrap(), sum.iter().max().unwrap(), sum.iter().max().unwrap() - sum.iter().min().unwrap());

    }

    pub fn polymerization_3 () {
        let contents = get_input();

        let (line, template) = contents.split_once("\r\n\r\n").unwrap();
        let line = line.to_string();

        let mut template: Vec<&str> = template.lines().collect();
        template.sort();

        let template_vec : Vec<Vec<_>>= template.iter().map(|x| {
            x.split(" -> ").collect()
        }).collect();



        let mut combination_counter_vec: Vec<i128> = vec![0; template_vec.len()];
        let mut char_counter_vec : HashMap<char, i128> = HashMap::new();

        let mut line_iter = line.chars();
        let line_iter_2 = line.chars().skip(1);

        for char_2 in line_iter_2 {
            let char_1 = line_iter.next().unwrap();
            char_counter_vec.insert(char_1, 0);


            for (i, combination) in template_vec.iter().enumerate() {
                if combination[0].chars().nth(0).unwrap() == char_1 && combination[0].chars().nth(1).unwrap() == char_2 {
                    combination_counter_vec[i] += 1;
                }
            }
        }

        char_counter_vec.insert(line_iter.next().unwrap(), 0);

        for char in line.chars() {
            char_counter_vec.insert(char, 1 + char_counter_vec[&char]);
        }

        println!("line: {}", line);
        println!("template_ vec: {:?}", template_vec);
        println!("combination_counter_vec: {:?}", combination_counter_vec);
        println!("char_counter_vec: {:?}", char_counter_vec);

        for _ in 0..40 {
            let combination_clone = combination_counter_vec.clone();
            let combination_iter = combination_clone.iter().enumerate();

            combination_counter_vec = combination_counter_vec.iter().map(|_| 0).collect();

            for (i2, number) in combination_iter.clone() {
                if *number != 0 {

                    let new_char = template_vec[i2][1].chars().nth(0).unwrap();
                    *char_counter_vec.entry(new_char).or_insert(0) += number;

                    let char_1 = template_vec[i2][0].chars().nth(0).unwrap();
                    let char_2 = template_vec[i2][0].chars().nth(1).unwrap();

                    for (i, combination) in template_vec.iter().enumerate() {
                        if combination[0].chars().nth(0).unwrap() == char_1 && combination[0].chars().nth(1).unwrap() == new_char {
                            combination_counter_vec[i] += number;
                        }
                        if combination[0].chars().nth(0).unwrap() == new_char && combination[0].chars().nth(1).unwrap() == char_2 {
                            combination_counter_vec[i] += number;
                        }
                    }
                }
            }
        }

        println!("end value: max - min: {}", char_counter_vec.iter().max_by_key(|x| x.1 ).unwrap().1 - char_counter_vec.iter().min_by_key(|x| x.1).unwrap().1);
    }

}