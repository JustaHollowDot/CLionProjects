use std::collections::VecDeque;
use std::io::Read;
use std::ptr::hash;
use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn syntax_sorting_corrupt () {
        let mut input = std::io::stdin();
        let mut string = String::new();
        input.read_to_string(&mut string).unwrap();

        let mut map = HashMap::new();
        map.insert( ']','[');
        map.insert(')', '(');
        map.insert('}', '{');
        map.insert('>', '<');

        let mut value_map: HashMap<char, i32> = HashMap::new();
        value_map.insert(')', 3);
        value_map.insert( ']',57);
        value_map.insert('}', 1197);
        value_map.insert('>', 25137);

        let mut sum = 0;


        'outer: for line in string.lines() {

            let mut queue = VecDeque::new();

            for char in line.chars() {
                match char {
                    '[' => queue.push_back(char),
                    '{' => queue.push_back(char),
                    '(' => queue.push_back(char),
                    '<' => queue.push_back(char),
                    _ => {
                        let case = queue.pop_back().unwrap();

                        if case != map[&char] {
                            println!("{}", char);

                            sum += value_map[&char];

                            continue 'outer
                        }
                    }
                }
            }
        }


        println!("{}", sum);

    }

    pub fn syntax_sorting_incomplete () {
        let mut input = std::io::stdin();
        let mut string = String::new();
        input.read_to_string(&mut string).unwrap();

        let mut map = HashMap::new();
        map.insert( ']','[');
        map.insert(')', '(');
        map.insert('}', '{');
        map.insert('>', '<');

        let mut value_map: HashMap<char, i32> = HashMap::new();
        value_map.insert('(', 1);
        value_map.insert( '[', 2);
        value_map.insert('{', 3);
        value_map.insert('<', 4);

        let mut sum = vec![];


        'outer: for line in string.lines() {

            let mut queue = VecDeque::new();

            for char in line.chars() {
                match char {
                    '[' => queue.push_back(char),
                    '{' => queue.push_back(char),
                    '(' => queue.push_back(char),
                    '<' => queue.push_back(char),
                    _ => {
                        let case = queue.pop_back().unwrap();
                        if case != map[&char] {
                            continue 'outer
                        }
                    }
                }
            }

            let mut temp_sum = 0;

            if !queue.is_empty() {
                println!("hei\n{}\n{:?}", line, queue);

                for char in queue.iter().rev() {
                    temp_sum *= 5;
                    temp_sum += value_map[&char] as i128;
                }
            }

            println!("{}", temp_sum);

            sum.push(temp_sum)
        }

        sum.sort();

        println!("{:?}", sum);
        println!("{:?}", sum[sum.len() / 2]);

    }
}