use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};

use rand::Rng;
use scanf::sscanf;

fn read_input() -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut from, mut tos) = (String::new(), String::new());
        if sscanf!(&line, "{}: {}", from, tos).is_ok() {
            let to_nodes = tos
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            result
                .entry(from.clone())
                .or_insert(Vec::new())
                .extend(to_nodes.clone());

            for to_node in &to_nodes {
                result
                    .entry(to_node.clone())
                    .or_insert(Vec::new())
                    .push(from.clone());
            }
        }
    }

    result
}

fn connected_components_sizes(connections: &HashMap<String, Vec<String>>) -> Vec<i32> {
    let mut visited = HashSet::new();

    let mut result = Vec::new();

    for start in connections.keys() {
        if visited.contains(start) {
            continue;
        }

        let mut q = VecDeque::new();
        q.push_back(start);
        visited.insert(start);

        let mut cnt = 0;

        while !q.is_empty() {
            let current = q.pop_back().unwrap();
            cnt += 1;

            for next in connections[current].iter() {
                if !visited.contains(next) {
                    visited.insert(next);
                    q.push_back(next);
                }
            }
        }

        result.push(cnt);
    }
    result
}

fn random_element<T>(v: &Vec<T>) -> &T {
    let idx = rand::thread_rng().gen_range(0..v.len());
    return &v[idx];
}

fn find_path(
    start: &String,
    end: &String,
    connections: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    let mut q = VecDeque::new();
    q.push_back((start, vec![start]));

    let mut visited = HashSet::new();
    visited.insert(start);

    while !q.is_empty() {
        let (current, path) = q.pop_front().unwrap();
        if current == end {
            let path_clone = path.iter().map(|&p| p.clone()).collect();
            return Some(path_clone);
        }

        for next in &connections[current] {
            if !visited.contains(next) {
                let mut new_path = path.clone();
                new_path.push(next);
                visited.insert(next);
                q.push_back((next, new_path));
            }
        }
    }
    None
}

fn remove_element<T: std::cmp::PartialEq>(v: &mut Vec<T>, val: T) {
    if let Some(idx) = v.iter().position(|el| *el == val) {
        v.remove(idx);
    }
}

fn remove_path_edges(path: &Vec<String>, connections: &mut HashMap<String, Vec<String>>) {
    for i in 0..path.len() - 1 {
        let (from, to) = (&path[i], &path[i + 1]);
        remove_element(connections.get_mut(from).unwrap(), to.clone());
        remove_element(connections.get_mut(to).unwrap(), from.clone());
    }
}

fn connected_components_if_removed(connections: &HashMap<String, Vec<String>>) -> Vec<i32> {
    let nodes = connections.keys().collect::<Vec<_>>();

    loop {
        let (start, end) = (random_element(&nodes), random_element(&nodes));

        let mut connections = connections.clone();
        for _ in 0..3 {
            let path = find_path(start, end, &connections).unwrap();
            remove_path_edges(&path, &mut connections);
        }

        if let None = find_path(start, end, &connections) {
            return connected_components_sizes(&connections);
        }
        // else, we've chosen 2 nodes that would be in the same component once (any) 3 edges are removed
        // so try again
    }
}
fn main() {
    let input = read_input();
    let sizes = connected_components_if_removed(&input);
    let result = sizes.iter().product::<i32>();

    println!("{result}");
}
