use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        println!("{strs:?}");

        let mut start_chars = "".to_string();
        let mut common_chars = "".to_string();
        println!("{start_chars}");


        for char in strs[0].chars() {
            println!("{char}");

            start_chars += &char.to_string();

            let mut test = true;
            for str in &strs[1..] {
                if !str.starts_with(&start_chars) {
                    test = false;
                }
            }

            if test {
                common_chars = (*start_chars).to_string();
            }
        }

        return common_chars.to_string();
    }
}