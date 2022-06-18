use crate::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {

        println!("{:?}", nums);

        let mut valid_triples: Vec<Vec<i32>> = vec![];

        for (i, num) in nums.iter().enumerate() {
            for (i2, num2) in nums.iter().enumerate() {
                if i2 == i {
                    continue;
                }
                for (i3, num3) in nums.iter().enumerate() {
                    if i3 == i || i3 == i2 {
                        continue;
                    }

                    if num + num2 + num3 == 0 {
                        let mut triple: Vec<i32> = vec![*num, *num2, *num3];
                        triple.sort();

                        let mut bool = true;

                        let mut test = true;
                        for valid_triple in &valid_triples {
                            if valid_triple == &triple && test == true{
                                bool = false;
                            } else if valid_triple == &triple && test == false {
                                test = true;
                            }

                        }

                        if bool == true {
                            valid_triples.push(triple);
                        }

                    }
                }
            }
        }

        println!("{:?}", valid_triples);

        /*

        let mut bool = false;

        let mut remove_index = vec![];

        'outer: for (i, valid_triple) in valid_triples.iter().enumerate() {

            for (i2, triple) in valid_triples.iter().enumerate() {

                if !(i >= i2) && valid_triple == triple {
                    remove_index.push(i2);
                    continue 'outer;
                }
            }
        }

        remove_index.sort();
        println!("{:?}", remove_index);


        let mut counter = 0;

        for kake in remove_index {
            valid_triples.remove(kake - counter);

            counter += 1;
        }


        println!("{:?}", valid_triples);

         */

        return valid_triples;
    }
}