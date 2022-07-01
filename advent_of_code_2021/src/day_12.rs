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

        edges.sort();

        for edge in &edges {
            adjacency_map[edge.0].push(edge.1);
            adjacency_map[edge.1].push(edge.0);
        }

        println!("{:?}", nodes);
        println!("{:?}", edges);
        println!("{:?}", adjacency_map);





        let sum = Solution::traverse_map(0, &adjacency_map, &mut vec![(false, false); nodes.len()], &nodes);

        println!("{}", sum);
    }

    fn traverse_map (current: usize, adjacency_map: &Vec<Vec<usize>>, visited_array: &mut Vec<(bool, bool)>, nodes: &Vec<&str>) -> i32 {
        for (i, node) in nodes.iter().enumerate() {
            if node.chars().any(|x| x.is_uppercase()) {
                visited_array[i].0 = true;
            }
        }

        if visited_array[current].0 == false && visited_array[current].1 == true {
            return 0;
        }

        println!("{}", nodes[current]);

        if nodes[current] == "end" {
            return 1;
        }


        visited_array[current].1 = true;

        let mut sum = 0;
        for adjacency in &adjacency_map[current] {
            sum += Solution::traverse_map(*adjacency, adjacency_map, visited_array, nodes)
        }


        return sum;
    }
}