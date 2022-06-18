use crate::Solution;

macro_rules! log_of {
    ($val:expr, $base:expr, $type:ty) => {
         ($val as f32).log($base) as $type
    }
}

impl Solution {
    pub fn is_palindrome (x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let digits = log_of!(x, 10., usize);

        for i in 0..digits / 2 + 1 {
            println!("{} {}", (x / (10_i32.pow(i as u32))) % 10, (x / (10_i32.pow((digits - i) as u32))) % 10);

            let a = (x / (10_i32.pow(i as u32))) % 10;
            let b = (x / (10_i32.pow((digits - i) as u32))) % 10;

            if a != b {
                return false;
            }
        }

        return true
    }
}