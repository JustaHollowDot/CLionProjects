use std::collections::{HashSet, VecDeque};
use std::ffi::c_void;
use std::fs::read_to_string;
use std::ops::Index;
use itertools::Itertools;
use crate::{Solution};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);
#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}

/*
pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex, is_capitalized: Vec<bool>, nodes: Vec<&str>) -> i32 {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut sum = 0;
    // While there is an element in the queue
    // get the first element of the vertex queue
    while let Some(current_vertex) = queue.pop_front() {
        // Added current vertex in the history of visiteds vertex
        history.push(current_vertex.value());
        println!("{}", nodes[current_vertex.value() as usize]);
        println!("{} : {}", current_vertex.value(), objective.value());
        println!(" ");
        // Verify if this vertex is the objective
        if current_vertex == objective {
            // Return the Optional with the history of visiteds vertex
            sum += 1;
            println!("{:?}", history);
            history = Vec::new();
        }
        // For each over the neighbors of current vertex
        for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
            // Insert in the HashSet of visiteds if this value not exist yet
            if !is_capitalized[neighbor.value() as usize] {
                queue.push_front(neighbor);
            } else {
                if visited.insert(neighbor) {
                    // Add the neighbor on front of queue
                    queue.push_front(neighbor);
                }
            }
        }
    }
    // If all vertex is visited and the objective is not found
    // return 0
    return sum
}
 */

impl Solution {
    /*
    pub fn amount_of_paths_with_structs () {
        let contents = read_to_string("input").unwrap();
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
        let mut node_index = Vec::new();
        for (i, node) in nodes.iter().enumerate() {
            node_index.push(i as u32);
        }
        let graph = Graph::new(
            node_index.into_iter().map(|v: u32 | v.into()).collect(),
            edges.into_iter().map(|e| {
                let x = (e.0 as u32, e.1 as u32);
                x.into()
            }).collect(),
        );
        let root: u32 = nodes.iter().position(|x| x == &"start").unwrap() as u32;
        let objective: u32 = nodes.iter().position(|x| x == &"end").unwrap() as u32;
        let mut is_capitalized = vec![false; nodes.len()];
        for (i, node) in nodes.iter().enumerate() {
            if node.chars().all(|x| x.is_ascii_uppercase()) {
                is_capitalized[i] = true;
            }
        }
        println!("{}", depth_first_search(&graph, root.into(), objective.into(), is_capitalized, nodes));
    }
     */


    pub fn amount_of_paths () {
        let contents = read_to_string("input").unwrap();

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

        let mut is_capitalized = vec![false; nodes.len()];

        for (i, node) in nodes.iter().enumerate() {
            if node.chars().all(|x| x.is_ascii_uppercase()) {
                is_capitalized[i] = true;
            }
        }


        edges.sort();

        for edge in &edges {
            adjacency_map[edge.0].push(edge.1);
            adjacency_map[edge.1].push(edge.0);
        }

        println!("{:?}", nodes);
        println!("{:?}", edges);
        println!("{:?}", adjacency_map);


        let mut test: Vec<Vec<usize>> = Vec::new();

        for node in &nodes {
            match node {
                &"start" => { continue }
                &"end" => { continue }
                _ => {
                    if node.chars().all(|x| x.is_ascii_uppercase()) {
                        continue
                    }
                }
            }

            let double = (*node, false);

            Solution::traverse_map(nodes.iter().position(|x| x == &"start").unwrap(), &adjacency_map, &nodes, Vec::new(), &is_capitalized, &mut test, double);
        }

        test = test.into_iter().filter(|x| !x.is_empty() ).collect();

        println!("stop");

    }

    fn traverse_map (current: usize, adjacency_map: &Vec<Vec<usize>>, nodes: &Vec<&str>, mut visited_array: Vec<usize>, is_capitalized: &Vec<bool>, test: &mut Vec<Vec<usize>>, mut double: (&str, bool)) -> Vec<usize> {

        if nodes[current] == "end" {
            visited_array.push(current);


            return visited_array.clone()
        }

        if is_capitalized[current] == false && visited_array.contains(&current) {

            if double.0 == nodes[current] && double.1 == true {
                return Vec::new();
            } else if double.0 == nodes[current] {
                double.1 = true;
            } else {
                return Vec::new();
            }
        }

        visited_array.push(current);

        let mut queue = VecDeque::new();

        for adjacency in &adjacency_map[current] {
            queue.push_front(adjacency);
        }

        let mut sum : Vec<usize> = Vec::new();

        while !queue.is_empty() {
            let next_index = queue.pop_front().unwrap();

            let temp = Solution::traverse_map(*next_index, adjacency_map, nodes, visited_array.clone(), is_capitalized, test, double);

            if !test.contains(&temp) {
                test.push(temp)
            }
        }

        return sum;
    }
}