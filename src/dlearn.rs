use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque, hash_map::Entry},
};

use itertools::Itertools;

pub fn search() {
    static INPUT: &str = r"
1-2
2-3
3-6
6-9
9-8
8-7
7-4
1-4
5-8
";
    let adj = INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.chars().next().unwrap(), b.chars().next().unwrap())
        })
        .collect_vec();

    let conn = adj.iter().fold(
        BTreeMap::new(),
        |mut conn: BTreeMap<char, BTreeSet<char>>, (a, b)| {
            conn.entry(*a).or_default().insert(*b);
            conn.entry(*b).or_default().insert(*a);
            conn
        },
    );

    // breadth first search
    let mut bfs = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((0, '1', '1'));
    while let Some((dist, par, node)) = q.pop_front() {
        if let Entry::Vacant(e) = bfs.entry(node) {
            e.insert((dist, par));
            q.extend(conn[&node].iter().map(|next| (1 + dist, node, *next)));
        }
    }
    println!("BFS");
    println!("{:?}", bfs.iter().sorted().format("\n"));

    // depth first search
    let mut dfs = HashMap::new();
    let mut q = Vec::new();
    q.push((0, '1', '1'));
    while let Some((dist, par, node)) = q.pop() {
        if let Entry::Vacant(e) = dfs.entry(node) {
            e.insert((dist, par));
            q.extend(conn[&node].iter().map(|next| (1 + dist, node, *next)));
        }
    }
    println!("DFS");
    println!("{:?}", dfs.iter().sorted().format("\n"));

    // dijkstra
    let mut dijk = HashMap::new();
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), '1', '1'));
    while let Some((Reverse(dist), par, node)) = q.pop() {
        if let Entry::Vacant(e) = dijk.entry(node) {
            e.insert((dist, par));
            q.extend(
                conn[&node]
                    .iter()
                    .map(|next| (Reverse(1 + dist), node, *next)),
            );
        }
    }
    println!("Dijkstra");
    println!("{:?}", dijk.iter().sorted().format("\n"));
}
