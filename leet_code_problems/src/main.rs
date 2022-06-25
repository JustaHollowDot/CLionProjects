#[allow(unused_variables)]
#[allow(dead_code)]

mod problem_3_sum;
mod zigzag_conversion;
mod two_sum;
mod reverse_integral;
mod palindrome_number;
mod longest_common_prefix;
mod container_with_most_water;

struct Solution {
}

fn main() {
    //let nums = vec![3,2,4];
    //let test = Solution::two_sum(nums, 6);

    //let nums = vec![-1,0,1,2,-1,-4];
    //let test = Solution::three_sum(nums);

    // let string = "AB".to_string();
    // let num_rows = 1;
    // let test = Solution::convert(string, num_rows);

    // let x = -2147483648;
    // let test = Solution::reverse(x);

    // let number = 121;
    // let test = Solution::is_palindrome(number);

    // let strs = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
    // let test = Solution::longest_common_prefix(strs);

    let height = vec![1,8,6,2,5,4,8,3,7];
    let test = Solution::max_area(height);


    println!("\n\n{:?}", test);
    }
