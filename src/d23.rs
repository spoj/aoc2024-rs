use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

pub static SAMPLE: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;
pub static INPUT: &str = include_str!("../data/d23.txt");

pub fn part1(input: &str) {
    let adj = input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .collect_vec();
    let mut conn: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        let (a, b) = l.split_once('-').unwrap();
        conn.entry(a).or_default().insert(b);
        conn.entry(b).or_default().insert(a);
    });
    let conn = conn;
    let mut three_cycles: HashSet<Vec<&str>> = HashSet::new();

    for (a, b) in adj {
        let left_set = &conn[&a];
        let right_set = &conn[&b];
        for i in left_set.intersection(right_set) {
            let mut three = vec![a, b, i];
            three.sort();
            three_cycles.insert(three);
        }
    }
    let ans = three_cycles
        .into_iter()
        .filter(|v| v.iter().any(|s| s.starts_with('t')))
        .count();
    dbg!(ans);
}

pub fn part2(input: &str) {
    let adj = input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .collect_vec();
    let mut conn: HashMap<&str, BTreeSet<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        let (a, b) = l.split_once('-').unwrap();
        conn.entry(a).or_default().insert(b);
        conn.entry(b).or_default().insert(a);
    });
    let conn = conn;
    let nodes = conn.keys().collect_vec();

    let mut islands: BTreeSet<BTreeSet<&str>> = Default::default();
    let mut islands_next: BTreeSet<BTreeSet<&str>> = Default::default();

    // initialize with adjacent list
    adj.iter().for_each(|(a, b)| {
        islands_next.insert(BTreeSet::from([*a, *b]));
    });

    while !islands_next.is_empty() {
        islands = islands_next;
        islands_next = islands
            .iter()
            .flat_map(|island| {
                let eligible = nodes
                    .iter()
                    .filter(|candidate| {
                        island
                            .iter()
                            .all(|member| conn[**candidate].contains(member))
                    })
                    .collect_vec();
                eligible.into_iter().map(move |new_node| {
                    let mut new_island = island.clone();
                    new_island.insert(new_node);
                    new_island
                })
            })
            .collect();
    }
    let ans = islands.first().unwrap();
    let day23_part2 = format!("{}", ans.iter().format(","));
    dbg!(day23_part2);
}
