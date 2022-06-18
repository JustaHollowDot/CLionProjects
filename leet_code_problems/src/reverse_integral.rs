use crate::Solution;

macro_rules! log_of {
    ($val:expr, $base:expr, $type:ty) => {
         ($val as f32).log($base) as $type
    }
}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let abs_x = (x as i64).abs();
        let digits = log_of!(abs_x, 10.,usize) + 1;

        println!("number: {}, digits: {}", x, digits);

        let mut new_number:i64 = 0;

        for i in 0..digits {
            let test:i64 = ((x / 10_i32.pow(i as u32)) % 10) as i64;

            println!("{}", test);


            new_number += (test * 10_i64.pow((digits - i - 1) as u32)) as i64;

            if new_number as i32 as i64 != new_number {
                return 0;
            }


            println!("{}", new_number)


        }

        println!("{}", new_number);

        return new_number as i32;
    }
}