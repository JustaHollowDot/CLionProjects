use std::cmp::{max, Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use std::u32::MAX;
use itertools::Itertools;
use crate::{get_input, Solution};



fn build_board() -> Vec<Vec<u32>> {
    let contents = get_input();
    let contents : Vec<Vec<u32>> = contents.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    for content in &contents {
        println!("{:?}", content);
    }

    let mut new_vec: Vec<Vec<u32>> = vec![vec![0; contents.len() * 5]; contents.len() * 5];

    for i  in 0..5 {
        for j in 0..5 {
            for (k, number_vec) in contents.iter().enumerate() {
                for (l, number) in number_vec.iter().enumerate() {

                    let new_number = if *number + i + j > 9 {
                        *number + i + j - 9
                    } else {
                        *number + i + j
                    };




                    new_vec[contents.len() * i as usize + k][contents.len() * j as usize + l] = new_number
                }
            }

            println!("\n\n\n\n\n");
        }
    }

    for line in &new_vec {
        println!("{:?}", line);
    }
    println!("{}", new_vec.len());

    new_vec

}




fn find_shortest_path(source: (usize, usize), destination: (usize, usize), graph: Vec<Vec<u32>>) {
    println!("{} -> {}", graph[source.0][source.1], graph[destination.0][destination.1]);

    let mut queue: VecDeque<(u32, (usize, usize))> = VecDeque::new();

    queue.push_front((0, source));

    while let Some(tuple_index) = queue.pop_front() {
        if tuple_index.0 > 5 {
            return;
        }



        traverse(tuple_index, graph.clone(), &mut queue);

        if queue.front().unwrap().1 == destination {
            return;
        }

        println!("{:?}", tuple_index);
        println!("{:?}", queue);

        let mut vector = Vec::from(queue.clone());

        vector.sort();
        println!("{:?}", vector);

        queue = VecDeque::from(vector);
    }


}

fn traverse(tuple_index: (u32, (usize, usize)), graph: Vec<Vec<u32>>, queue: &mut VecDeque<(u32, (usize, usize))>) {
    let source = tuple_index.1;
    if source.0 != 0 {
        let sum = tuple_index.0 + graph[source.0 - 1][source.1];
        queue.push_front( (sum, (source.0 - 1, source.1)));

    }
    if source.0 != graph.len() - 1 {
        let sum = tuple_index.0 + graph[source.0 + 1][source.1];
        queue.push_front( (sum, (source.0 + 1, source.1)));

    }

    if source.1 != 0 {
        let sum = tuple_index.0 + graph[source.0][source.1 - 1];
        queue.push_front( (sum, (source.0, source.1 - 1)));

    }
    if source.1 != graph.len() - 1 {
        let sum = tuple_index.0 + graph[source.0][source.1 + 1];
        queue.push_front( (sum, (source.0, source.1 + 1)));

    }

}



fn find_shortest_path_v2(visited: Vec<(usize, usize)>, graph: Vec<Vec<u32>>, destination: (usize, usize)) {
    let current = visited.iter().last().unwrap();

    println!("{} -> {}", graph[current.0][current.1], graph[destination.0][destination.1]);

    let mut paths: Vec<(u32, Vec<(usize,usize)>)> = Vec::new();

    paths.push((0, vec![current.clone()]));

    for path in &paths {
        println!("{:?}", path);
    }

    loop {
        traverse_v2(&graph, &mut paths);

        if paths.first().unwrap().1.last().unwrap() == &destination {
            break
        }

    }
}

fn traverse_v2(graph: &Vec<Vec<u32>>, paths: &mut Vec<(u32, Vec<(usize, usize)>)>) {
    let mut index_vec = paths.first().unwrap().1.clone();
    let current_index = index_vec.last().unwrap().clone();

    if current_index.0 != 0 {
        if !index_vec.contains(&(current_index.0 - 1, current_index.1)) {
            let sum = paths.first().unwrap().0 + graph[current_index.0 - 1][current_index.1];


            index_vec.push((current_index.0 - 1, current_index.1));

            paths.push((sum, index_vec.clone()));

            index_vec.pop();
        }
    }

    if current_index.0 != graph.len() - 1 {
        if !index_vec.contains(&(current_index.0 + 1, current_index.1)) {
            let sum = paths.first().unwrap().0 + graph[current_index.0 + 1][current_index.1];
            index_vec.push((current_index.0 + 1, current_index.1));

            paths.push((sum, index_vec.clone()));

            index_vec.pop();
        }
    }

    if current_index.1 != 0 {
        if !index_vec.contains(&(current_index.0, current_index.1 - 1)) {
            let sum = paths.first().unwrap().0 + graph[current_index.0][current_index.1 - 1];
            index_vec.push((current_index.0, current_index.1 - 1));

            paths.push((sum, index_vec.clone()));

            index_vec.pop();
        }
    }

    if current_index.1 != graph.len() - 1 {
        if !index_vec.contains(&(current_index.0, current_index.1 + 1)) {
            let sum = paths.first().unwrap().0 + graph[current_index.0][current_index.1 + 1];
            index_vec.push((current_index.0, current_index.1 + 1));

            paths.push((sum, index_vec.clone()));

            index_vec.pop();
        }

        paths.remove(0);

        paths.sort();
    }
}



fn find_shortest_path_v3(graph: Vec<Vec<u32>>, visited: Vec<(u32, (usize, usize))>, destination: (usize, usize)) {
    let current = visited.last().unwrap().1;

    let mut paths = vec![visited];



    loop {
        traverse_v3(&graph, &mut paths);

        println!("cost: {:?}", paths.first().unwrap().last().unwrap().0 );

        if paths.first().unwrap().last().unwrap().1 == destination {
            return;
        }

    }

}

fn traverse_v3(graph: &Vec<Vec<u32>>, paths: &mut Vec<Vec<(u32, (usize, usize))>>) {
    let mut index_vec = paths.first().unwrap().clone();
    let current_index = index_vec.last().unwrap().1;
    let current_sum = index_vec.last().unwrap().0;

    let mut next_indexes: Vec<(usize, usize)> = Vec::new();

    if current_index.0 != 0 {
        next_indexes.push((current_index.0 - 1, current_index.1));
    }
    if current_index.0 != graph.len() - 1 {
        next_indexes.push((current_index.0 + 1, current_index.1));
    }
    if current_index.1 != 0 {
        next_indexes.push((current_index.0, current_index.1 - 1));
    }
    if current_index.1 != graph.len() - 1 {
        next_indexes.push((current_index.0, current_index.1 + 1));
    }




    for next_index in next_indexes {
        if !index_vec.iter().any(|(sum, x_y)| x_y ==  &next_index) {
            let next_sum = current_sum + graph[next_index.0][next_index.1];




            let paths_clone = paths.clone();
            let mut paths_iter = paths_clone.iter();

            let test = paths_iter.position(|x| x.iter().any(|(sum, index)| index == &next_index));

            if test != None {


                let vec_index = paths[test.unwrap()].iter().position(|(sum, x_y)| x_y == &next_index).unwrap();

                if paths[test.unwrap()][vec_index].0 > next_sum {


                    paths.remove(test.unwrap());

                    let push_index = paths.iter().position(|x| x.last().unwrap().0 >= next_sum);

                    let mut new_vec_index = index_vec.clone();
                    new_vec_index.push((next_sum, next_index));

                    if push_index != None {
                        paths.insert(push_index.unwrap(), new_vec_index)
                    } else {
                        paths.push(new_vec_index)
                    }

                }
            } else {
                let push_index = paths.iter().position(|x| x.last().unwrap().0 >= next_sum);

                let mut new_vec_index = index_vec.clone();
                new_vec_index.push((next_sum, next_index));

                if push_index != None {
                    paths.insert(push_index.unwrap(), new_vec_index)
                } else {
                    paths.push(new_vec_index)
                }
            }
        }
    }

    paths.remove(0);
}

fn find_shortest_path_v4(graph: Vec<Vec<u32>>, cost_vec: &mut Vec<Vec<u32>>, destination: (usize, usize)) {
    let mut queue:VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_front((0,0));

    while let Some(current) = queue.pop_front() {
        traverse_v4(graph.clone(), cost_vec, current, &mut queue);

        // println!("{:?}", current);
    }
}

fn traverse_v4(graph: Vec<Vec<u32>>, cost_vec: &mut Vec<Vec<u32>>, current_index: (usize, usize), queue: &mut VecDeque<(usize, usize)>) {
    let current_sum = if current_index == ( 0, 0 ) {
        0
    } else {
        cost_vec[current_index.0][current_index.1]
    };

    let mut next_indexes: Vec<(usize, usize)> = Vec::new();

    if current_index.0 != 0 {
        next_indexes.push((current_index.0 - 1, current_index.1));
    }
    if current_index.0 != graph.len() - 1 {
        next_indexes.push((current_index.0 + 1, current_index.1));
    }
    if current_index.1 != 0 {
        next_indexes.push((current_index.0, current_index.1 - 1));
    }
    if current_index.1 != graph.len() - 1 {
        next_indexes.push((current_index.0, current_index.1 + 1));
    }


    for next_index in next_indexes {


        if cost_vec[next_index.0][next_index.1] > current_sum + graph[next_index.0][next_index.1] {
            cost_vec[next_index.0][next_index.1] = current_sum + graph[next_index.0][next_index.1];

            queue.push_front( next_index );

        }

    }
}

fn create_edges(graph: Vec<Vec<u32>>, current_index: (usize, usize)) -> Vec<(usize, usize)> {
    let mut next_indexes: Vec<(usize, usize)> = Vec::new();

    if current_index.0 != 0 {
        next_indexes.push((current_index.0 - 1, current_index.1));
    }
    if current_index.0 != graph.len() - 1 {
        next_indexes.push((current_index.0 + 1, current_index.1));
    }
    if current_index.1 != 0 {
        next_indexes.push((current_index.0, current_index.1 - 1));
    }
    if current_index.1 != graph.len() - 1 {
        next_indexes.push((current_index.0, current_index.1 + 1));
    }

    next_indexes
}


impl Solution {
    pub fn chiton_part_1 () {
        let contents = get_input();
        let contents : Vec<Vec<u32>> = contents.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

        for content in &contents {
            println!("{:?}", content);
        }

        find_shortest_path((0,0), (contents.len()-1, contents.len()-1), contents);

    }

    pub fn chiton_part_1_version_2 () {
        let contents = get_input();
        let contents : Vec<Vec<u32>> = contents.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

        for content in &contents {
            println!("{:?}", content);
        }

        find_shortest_path_v2(vec![(0,0)], contents.clone(),(contents.len()-1, contents.len()-1));
    }

    pub fn chiton_part_1_v3 () {
        let contents = build_board();



        for content in &contents {
            println!("{:?}", content);
        }

        find_shortest_path_v3(contents.clone(), vec![(0, (0, 0))], (contents.len() - 1, contents.len() - 1));




    }


    pub fn chiton_part_1_v4() {
        let contents = get_input();
        let contents : Vec<Vec<u32>> = contents.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
        let mut cost_vec : Vec<Vec<u32>> = vec![vec![MAX; contents.len()]; contents.len()];

        cost_vec[0][0] = 0;

        for content in &contents {
            println!("{:?}", content);
        }

        println!(" ");
        find_shortest_path_v4(contents.clone(), &mut cost_vec, (contents.len() - 1, contents.len() - 1));

        cost_vec.iter().for_each(|x| println!("{:?}", x));

        println!("{}", cost_vec[contents.len() - 1][contents.len() - 1]);



    }

    pub fn chiton_part_1_v5() {
        let contents = build_board();

        let mut cost_vec : Vec<Vec<u32>> = vec![vec![MAX; contents.len()]; contents.len()];

        cost_vec[0][0] = 0;

        for content in &contents {
            println!("{:?}", content);
        }

        let destination = (contents.len() - 1, contents.len() - 1);

        let mut heap = BinaryHeap::new();
        heap.push(State{cost: 0, position: (0, 0)});
        println!("{:?}", heap);

        let mut visited = Vec::new();

        while let Some(State { cost, position }) = heap.pop() {
            println!("{}", cost);
            if position == destination {
                println!("{}", cost);
            }

            if visited.contains(&position) {
                continue
            } else {
                visited.push(position);
            }

            let new_nodes = create_edges(contents.clone(), position);

            for node in new_nodes {
                let next = State{ cost: cost + contents[node.0][node.1] as usize, position: node};

                if next.cost < cost_vec[next.position.0][next.position.1] as usize {
                    heap.push(next);

                    cost_vec[next.position.0][next.position.1] = next.cost as u32;
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (usize, usize)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

