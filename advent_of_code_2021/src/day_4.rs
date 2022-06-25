use std::fs;
use crate::Solution;

impl Solution {
    pub fn bingo () {
        let contents = fs::read_to_string("input").unwrap();

        let lottery_numbers = contents.split_once("\r\n").unwrap().0;
        let lottery_numbers : Vec<&str> = lottery_numbers.split(",").collect();

        let mut temp : Vec<i32> = vec![];
        for lottery_number in lottery_numbers {
            temp.push(lottery_number.parse().unwrap());
        }

        let lottery_numbers = temp;
        println!("lottery_numbers: {lottery_numbers:?}");

        let mut bingo_boards: Vec<&str> = contents.split("\r\n\r\n").collect::<Vec<&str>>();
        bingo_boards.remove(0);

        let mut test : Vec<Vec<&str>> = vec![];
        for bingo_board in bingo_boards {
            test.push(bingo_board.split("\r\n").collect())
        }

        let mut board_numbers : Vec<Vec<Vec<&str>>> = vec![];

        for i in 0..test.len() {
            board_numbers.push(vec![]);

            for line in &test[i] {
                let kake : Vec<&str> = line.split_whitespace().collect();

                board_numbers[i].push(kake);
            }
        }

        let mut temp : Vec<Vec<Vec<i32>>>  = vec![];

        for (i, board) in board_numbers.iter().enumerate() {
            temp.push(vec![]);
            for (i2, row) in board.iter().enumerate() {
                temp[i].push(vec![]);
                for number in row {
                    temp[i][i2].push(number.parse::<i32>().unwrap());
                }
            }
        }

        let board_numbers : Vec<Vec<Vec<i32>>> = temp;

        println!("board_numbers: {board_numbers:?}");


        let mut is_bingo : Vec<Vec<Vec<bool>>> = vec![];

        for (i, board) in board_numbers.iter().enumerate() {
            is_bingo.push(vec![]);
            for (i2, row) in board.iter().enumerate() {
                is_bingo[i].push(vec![]);
                for (i3, number) in row.iter().enumerate() {
                    is_bingo[i][i2].push(false);
                }
            }
        }

        'outer: for lottery_number in lottery_numbers {
            for (i, board) in board_numbers.iter().enumerate() {
                for (i2, row) in board.iter().enumerate() {
                    for (i3, number) in row.iter().enumerate() {
                        if *number == lottery_number {
                            is_bingo[i][i2][i3] = true;
                        }
                    }
                }
            }

            for (i, board_number) in is_bingo.iter().enumerate() {
                for row in board_number {
                    let mut counter = 0;
                    for number in row {
                        if *number == true {
                            counter += 1;

                        }
                    }

                    if counter == 5 {

                        let mut kake = 0;
                        let mut counter = 0;
                        for (i2, row) in board_numbers[i].iter().enumerate() {
                            for (i3, number) in row.iter().enumerate() {
                                if !is_bingo[i][i2][i3] {
                                    counter += number;
                                    kake = *number;
                                }
                            }
                        }

                        println!("{}", counter * lottery_number);

                        break 'outer
                    }
                }
            }

            for (i, board) in is_bingo.iter().enumerate() {
                let columns = board.len();
                let rows = board[0].len();

                let iter = (0..rows).map(|row_idx| board.iter().flatten().skip(row_idx).step_by(columns));

                for (row_idx, row_values) in iter.enumerate() {
                    let mut counter = 0;
                    for (column_idx, value) in row_values.enumerate() {

                        if *value == true {
                            counter += 1;
                        }
                    }

                    if counter == 5 {

                        let mut counter = 0;
                        for (i2, row) in board_numbers[i].iter().enumerate() {
                            for (i3, number) in row.iter().enumerate() {
                                if !is_bingo[i][i2][i3] {
                                    counter += number;
                                }
                            }
                        }

                        println!("{}", counter * lottery_number);

                        break 'outer
                    }
                }
            }
        }
    }

    fn create_bingo_board () -> (Vec<Vec<Vec<i32>>>, Vec<i32>) {
        let contents = fs::read_to_string("input").unwrap();

        let lottery_numbers = contents.split_once("\r\n").unwrap().0;
        let lottery_numbers : Vec<&str> = lottery_numbers.split(",").collect();

        let mut temp : Vec<i32> = vec![];
        for lottery_number in lottery_numbers {
            temp.push(lottery_number.parse().unwrap());
        }

        let lottery_numbers = temp;
        println!("lottery_numbers: {lottery_numbers:?}");

        let mut bingo_boards: Vec<&str> = contents.split("\r\n\r\n").collect::<Vec<&str>>();
        bingo_boards.remove(0);

        let mut test : Vec<Vec<&str>> = vec![];
        for bingo_board in bingo_boards {
            test.push(bingo_board.split("\r\n").collect())
        }

        let mut board_numbers : Vec<Vec<Vec<&str>>> = vec![];

        for i in 0..test.len() {
            board_numbers.push(vec![]);

            for line in &test[i] {
                let kake : Vec<&str> = line.split_whitespace().collect();

                board_numbers[i].push(kake);
            }
        }

        let mut temp : Vec<Vec<Vec<i32>>>  = vec![];

        for (i, board) in board_numbers.iter().enumerate() {
            temp.push(vec![]);
            for (i2, row) in board.iter().enumerate() {
                temp[i].push(vec![]);
                for number in row {
                    temp[i][i2].push(number.parse::<i32>().unwrap());
                }
            }
        }

        println!("board_numbers: {temp:?}");

        return (temp, lottery_numbers)
    }

    fn create_bool_bingo_board (board_numbers: Vec<Vec<Vec<i32>>>) -> Vec<Vec<Vec<bool>>> {
        let mut is_bingo : Vec<Vec<Vec<bool>>> = vec![];

        for (i, board) in board_numbers.iter().enumerate() {
            is_bingo.push(vec![]);
            for (i2, row) in board.iter().enumerate() {
                is_bingo[i].push(vec![]);
                for (i3, number) in row.iter().enumerate() {
                    is_bingo[i][i2].push(false);
                }
            }
        }

        return is_bingo
    }

    pub fn last_bingo () {

        let (mut board_numbers, lottery_numbers) = Solution::create_bingo_board();

        let mut is_bingo = Solution::create_bool_bingo_board(board_numbers.clone());

        let mut index_to_skip = vec![];
        'outer: for lottery_number in lottery_numbers {
            for (i, board) in board_numbers.iter().enumerate() {
                for (i2, row) in board.iter().enumerate() {
                    for (i3, number) in row.iter().enumerate() {
                        if *number == lottery_number {
                            is_bingo[i][i2][i3] = true;
                        }
                    }
                }
            }



            for (i, board_number) in is_bingo.iter().enumerate() {

                if index_to_skip.contains(&i) {
                    continue
                }

                for row in board_number {
                    let mut counter = 0;
                    for number in row {
                        if *number == true {
                            counter += 1;

                        }
                    }

                    if counter == 5 {
                        if index_to_skip.contains(&i) {
                            continue
                        }

                        index_to_skip.push(i);

                        if index_to_skip.len() != board_numbers.len() {
                            continue
                        }

                            let mut kake = 0;
                            let mut counter = 0;
                            for (i2, row) in board_numbers[i].iter().enumerate() {
                                for (i3, number) in row.iter().enumerate() {
                                    if !is_bingo[i][i2][i3] {
                                        counter += number;
                                        kake = *number;
                                    }
                                }
                            }

                            println!("{:?}", board_numbers[i]);

                            println!("{:?}", board_numbers[i]);
                            println!("{:?}", is_bingo[i]);
                            println!("index_to_skip: {:?}", index_to_skip);

                            println!("2: {}", counter * lottery_number);



                            break 'outer
                    }
                }
            }

            for (i, board) in is_bingo.iter().enumerate() {
                if index_to_skip.contains(&i) {
                    continue
                }

                let columns = board.len();
                let rows = board[0].len();

                let iter = (0..rows).map(|row_idx| board.iter().flatten().skip(row_idx).step_by(columns));

                for (row_idx, row_values) in iter.enumerate() {
                    let mut counter = 0;
                    for (column_idx, value) in row_values.enumerate() {

                        if *value == true {
                            counter += 1;
                        }
                    }

                    if counter == 5 {
                        if index_to_skip.contains(&i) {
                            continue
                        }

                        index_to_skip.push(i);

                        if index_to_skip.len() != board_numbers.len() {
                            continue
                        }

                            let mut counter = 0;
                            for (i2, row) in board_numbers[i].iter().enumerate() {
                                for (i3, number) in row.iter().enumerate() {
                                    if !is_bingo[i][i2][i3] {
                                        counter += number;
                                    }
                                }
                            }

                            println!("\n{:?}", board_numbers[i]);
                            println!("{:?}", is_bingo[i]);
                            println!("index_to_skip: {:?}", index_to_skip);

                            println!("{} : {}", counter, lottery_number);
                            println!("1: {}", counter * lottery_number);

                            break 'outer
                    }
                }
            }
        }
    }
}