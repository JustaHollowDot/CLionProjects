use crate::Solution;

impl Solution {
    pub fn two_sum<'a>(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lenght = nums.len();

        let mut new_list : Vec<i32> = vec![0; lenght];
        println!("{:?}", new_list);

        let mut index = 0;
        for (i, el) in nums.iter().enumerate() {
            println!("{}, {}", el, i);


            new_list[i] = target - el;
        }


        println!("{:?}\n{:?}", new_list, nums);

        for (i, el) in new_list.iter().enumerate() {
            for (i2, num) in nums.iter().enumerate() {
                if el == num && i != i2 {
                    println!("dette er paret: {}, {}", nums[i], num);

                    println!("{} {} {} {}", i, i2, nums[i], num);
                    let test = i as i32;
                    let test2 = i2 as i32;
                    return vec![test, test2];
                }
            }
        }

        return nums;
    }
}