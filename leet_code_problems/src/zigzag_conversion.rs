use crate::Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        println!("string: {}, num_rows: {}", s, num_rows);

        let mut lines: Vec<String> = vec!["".to_string(); num_rows as usize];

        let mut increasing:bool = true;
        increasing = false;

        let mut counter = 0;

        if num_rows == 1 {
            return s;
        }

        for (i, char) in s.chars().enumerate() {
            println!("{} {}", char, i);

            lines[counter].push(char);

            if increasing == true && counter < (num_rows - 1) as usize {
                println!("{:?}", lines);

                counter += 1;
            } else if increasing == true && counter >= (num_rows - 1) as usize {
                println!("{:?}", lines);

                increasing = false;
                counter -= 1;

            } else if increasing == false && counter > 0 {
                println!("{:?}", lines);

                counter -= 1;
            } else {
                println!("{:?}", lines);

                increasing = true;
                counter += 1;
            }
        }

        println!("{:?}", lines);


        let final_string = lines.concat();
        println!("{}", final_string);

        return final_string;
    }
}