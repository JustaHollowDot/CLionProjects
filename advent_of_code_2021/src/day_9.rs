use std::fs::read_to_string;
use crate::Solution;


impl Solution {
    pub fn low_points () {
        let contents = read_to_string("input").unwrap();
        let contents : Vec<&str> = contents.lines().collect();

        let mut temp: Vec<Vec<i32>> = vec![];
        for (i, content) in contents.iter().enumerate() {

            temp.push(vec![]);
            for char in content.chars() {
                temp[i].push(char.to_digit(10).unwrap().try_into().unwrap());
            }
        }

        let contents : Vec<Vec<i32>> = temp;

        let mut all_points = vec![];

        for (i, content) in contents.iter().enumerate() {
            for (i2, number) in content.iter().enumerate() {
                all_points.push(Solution::return_lowest_point_and_index(number, i, i2, &contents, vec![vec![]]));
            }
        }

        all_points.sort();
        all_points.dedup();

        let mut sum = 0;
        for point in &all_points {
            if point.0 == 10 {
                continue
            }
            sum += point.0 + 1
        }

        println!("{}", sum);
    }

    fn return_lowest_point_and_index<'a>(number: &'a i32, i: usize, i2: usize, contents: &'a Vec<Vec<i32>>, mut previous_indexes: Vec<Vec<usize>>) -> (i32, usize, usize) {
        if i != 0_usize {
            if contents[(i - 1) as usize][i2] <= *number {
                if !previous_indexes.contains(&vec![i-1, i2]) {
                    previous_indexes.push(vec![i, i2]);
                    return Solution::return_lowest_point_and_index(&contents[(i - 1) as usize][i2], i - 1, i2, contents, previous_indexes)
                } else {
                    return (10, i, i2)
                }
            }
        }
        if i != (contents.len() - 1) as usize {
            if contents[(i + 1) as usize][i2] <= *number {
                if !previous_indexes.contains(&vec![i + 1, i2]) {
                    previous_indexes.push(vec![i, i2]);
                    return Solution::return_lowest_point_and_index(&contents[(i + 1) as usize][i2], i + 1, i2, contents, previous_indexes)
                } else {
                    return (10, i, i2)
                }
            }
        }
        if i2 != 0_usize {
            if contents[i][(i2 - 1) as usize] <= *number {
                if !previous_indexes.contains(&vec![ i, i2 - 1]) {
                    previous_indexes.push(vec![i, i2]);
                    return Solution::return_lowest_point_and_index(&contents[i][(i2 - 1) as usize], i, i2 - 1, contents, previous_indexes)
                } else {
                    return (10, i, i2)
                }
            }
        }
        if i2 != (contents[0].len() - 1) as usize {
            if contents[i][(i2 + 1) as usize] <= *number {
                if !previous_indexes.contains(&vec![i, i2 + 1]) {
                    previous_indexes.push(vec![i, i2]);
                    return Solution::return_lowest_point_and_index(&contents[i][(i2 + 1) as usize], i, i2 + 1, contents, previous_indexes)
                } else {
                    return (10, i, i2)
                }
            }
        }

        return (*number, i, i2)
    }


    pub fn basins () {
        let contents = read_to_string("input").unwrap();
        let contents : Vec<&str> = contents.lines().collect();

        let mut temp: Vec<Vec<i32>> = vec![];
        for (i, content) in contents.iter().enumerate() {

            temp.push(vec![]);
            for char in content.chars() {
                temp[i].push(char.to_digit(10).unwrap().try_into().unwrap());
            }
        }

        let contents : Vec<Vec<i32>> = temp;

        let mut all_points = vec![];

        for (i, content) in contents.iter().enumerate() {
            for (i2, number) in content.iter().enumerate() {
                all_points.push(Solution::return_lowest_point_and_index_with_basin_amount(number, i, i2, &contents, vec![], 0));
            }
        }

        all_points.sort();
        all_points.dedup();

        let mut vec = vec![];
        let mut vec_len = vec![];

        let mut sum = 0;
        let mut previous = (100000000000, 100000000000);

        for point in &all_points {
            if point.0 == 10 {
                continue
            }

            if (point.1, point.2) != previous {
                vec.sort();
                vec.dedup();
                println!("previous vector: {:?}, {}", vec, vec.len() + 1);

                vec_len.push(vec.len() + 1);

                vec = vec![];
            }


            if (point.1, point.2) != previous {
                sum += point.0 + 1;

                previous = (point.1, point.2);
            }


            println!("{:?}", point);

            for vector in &point.4 {
                if contents[vector[0]][vector[1]] == 9 {
                    continue
                }

                vec.push(vector);
            }
        }

        vec.sort();
        vec.dedup();

        println!("previous vector: {:?}, {}", vec, vec.len() + 1);

        vec_len.push(vec.len() + 1);

        println!("{}", sum);

        vec_len.sort();

        println!("{:?}", vec_len);

        let len = vec_len.len();

        let product_three_largest = vec_len[len - 1] * vec_len[len - 2] * vec_len[len - 3];

        println!("{}", product_three_largest);



    }

    fn return_lowest_point_and_index_with_basin_amount<'a>(number: &'a i32, i: usize, i2: usize, contents: &'a Vec<Vec<i32>>, mut previous_indexes: Vec<Vec<usize>>, mut basin_amount: i32) -> (i32, usize, usize, i32, Vec<Vec<usize>>) {
        if *number == 9 {
            basin_amount = 0;
        } else {
            basin_amount += 1;
        }

        if i != 0_usize {
            if contents[(i - 1) as usize][i2] < *number {
                if !previous_indexes.contains(&vec![i-1, i2]) {
                    previous_indexes.push(vec![i, i2]);
                    return Solution::return_lowest_point_and_index_with_basin_amount(&contents[(i - 1) as usize][i2], i - 1, i2, contents, previous_indexes, basin_amount)
                } else {
                    return (10, i, i2, basin_amount, previous_indexes)
                }
            }
        }
        if i != (contents.len() - 1) as usize {
            if contents[(i + 1) as usize][i2] < *number {
                if !previous_indexes.contains(&vec![i + 1, i2]) {
                    previous_indexes.push(vec![i, i2]);
                    return Solution::return_lowest_point_and_index_with_basin_amount(&contents[(i + 1) as usize][i2], i + 1, i2, contents, previous_indexes, basin_amount)
                } else {
                    return (10,  i, i2, basin_amount, previous_indexes)
                }
            }
        }
        if i2 != 0_usize {
            if contents[i][(i2 - 1) as usize] < *number {
                if !previous_indexes.contains(&vec![ i, i2 - 1]) {
                    previous_indexes.push(vec![i, i2]);
                    return Solution::return_lowest_point_and_index_with_basin_amount(&contents[i][(i2 - 1) as usize], i, i2 - 1, contents, previous_indexes, basin_amount)
                } else {
                    return (10,  i, i2, basin_amount, previous_indexes)
                }
            }
        }
        if i2 != (contents[0].len() - 1) as usize {
            if contents[i][(i2 + 1) as usize] < *number {
                if !previous_indexes.contains(&vec![i, i2 + 1]) {
                    previous_indexes.push(vec![i, i2]);
                    return Solution::return_lowest_point_and_index_with_basin_amount(&contents[i][(i2 + 1) as usize], i, i2 + 1, contents, previous_indexes, basin_amount)
                } else {
                    return (10, i, i2, basin_amount, previous_indexes)
                }
            }
        }

        return (*number, i, i2, basin_amount, previous_indexes)
    }
}