use std::fs::read_to_string;
use crate::Solution;

impl Solution {
    pub fn fold () {
        let contents = read_to_string("input").unwrap();

        let contents : Vec<&str> = contents.split("\r\n\r\n").collect();

        let fold = contents[1];

        let contents = contents[0];

        let mut points: Vec<(usize, usize)> = Vec::new();
        for content in contents.lines() {
            let numbers : Vec<&str> = content.split(",").collect();

            points.push( (numbers[0].parse::<usize>().unwrap(), numbers[1].parse::<usize>().unwrap()) );
        }

        println!("{:?}", points);


        let max_x = points.iter().map(|(x,y)| x ).max().unwrap();
        let max_y = points.iter().map(|(x,y)| y ).max().unwrap();


        println!("{} {}", max_x, max_y);


        let mut map = vec![vec![false; *max_x + 1]; *max_y + 1];



        points.iter().for_each(|(x, y)| map[*y][*x] = true);


        let fold_1 = fold.lines().collect::<Vec<&str>>()[0];

        println!("{}", fold_1);

        for fold_1 in fold.lines() {
            if fold_1.contains("x") {
                let fold_line: usize = fold_1.to_string().strip_prefix("fold along x=").unwrap().parse().unwrap();

                for mut line in &mut map{
                    while let Some(x) = line.iter().enumerate().position(|(i, x)| i > fold_line && *x == true) {
                        let difference = x - fold_line;
                        line[fold_line - difference] = true;
                        line[x] = false
                    }

                    while line.len() > fold_line {
                        line.pop();
                    }
                }
            } else {
                let fold_line: usize = fold_1.to_string().strip_prefix("fold along y=").unwrap().parse().unwrap();

                let map_clone = map.clone();

                let map_iter = map_clone.iter().enumerate();

                map_iter.for_each(|(i, line)| {
                    if i > fold_line {

                        line.iter().enumerate().for_each(|(i2, x)| {
                            if *x == true {
                                let difference = i - fold_line;

                                map[fold_line - difference][i2] = true;
                                map[i][i2] = false;
                            }
                        })
                    }



                });

                while map.len() > fold_line {
                    map.pop();
                }
            }
        }

        for line in &map {
            line.iter().for_each(|x| if *x == true {
                print!("{} ", *x as i8)
            } else {
                print!("  ");
            });
            println!(" ");
        }

        let mut sum = 0;

        map.iter().flatten().for_each(|x| {
            if *x == true { sum += 1 }
        });

        println!("sum: {}", sum);

    }
}