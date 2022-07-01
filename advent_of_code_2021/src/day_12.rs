use std::ffi::c_void;
use itertools::Itertools;
use crate::{create_vec_vec_uint, get_input, Solution};

impl Solution {
    pub fn amount_of_paths () {
        let contents = get_input();

        let mut nodes = Vec::new();

        for line in contents.lines() {
            let some_nodes : Vec<&str> = line.split("-").collect();

            for some_node in some_nodes {
                if !nodes.contains(&some_node) {
                    nodes.push(some_node);
                }
            }
        }

        let mut edges = Vec::new();

        for line in contents.lines() {
            let some_nodes : Vec<&str> = line.split("-").collect();

            edges.push((
                nodes.iter().position(|&x| x == some_nodes[0]).unwrap() ,
                nodes.iter().position(|&x| x == some_nodes[1]).unwrap()
            ));
        }

        let mut adjacency_map: Vec<Vec<usize>> = vec![vec![]; nodes.len()];

        let mut visited_array = vec![];
        let is_capitalized = vec![false; nodes.len()];

        edges.sort();

        for edge in &edges {
            adjacency_map[edge.0].push(edge.1);
            adjacency_map[edge.1].push(edge.0);
        }

        println!("{:?}", nodes);
        println!("{:?}", edges);
        println!("{:?}", adjacency_map);





        let sum = Solution::traverse_map(0, &adjacency_map, &nodes, &mut visited_array, &is_capitalized);

        println!("{}", sum);
    }

    fn traverse_map (current: usize, adjacency_map: &Vec<Vec<usize>>, nodes: &Vec<&str>, visited_array: &mut Vec<usize>, is_capitalized: &Vec<bool>) -> i32 {
        println!("{}", nodes[current]);

        if is_capitalized[current] == false && visited_array.contains(&current) {
            return 0;
        }

        if nodes[current] == "end" {
            println!("{:?}", visited_array);

            let visited_array : Vec<&usize> = visited_array.iter().filter(|_| false).collect();

            println!("all false: {:?}", visited_array);
            return 1;
        }


        visited_array.push(current);

        let mut sum = 0;
        for adjacency in &adjacency_map[current] {
            sum += Solution::traverse_map(*adjacency, adjacency_map, nodes, visited_array, is_capitalized)
        }


        return sum;
    }
}