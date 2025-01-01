#![warn(unused)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::LazyLock,
};

use itertools::Itertools;

use crate::answer;

pub static SAMPLE: &str = r#"029A
980A
179A
456A
379A
"#;
pub static INPUT: &str = include_str!("../data/d21.txt");

static UP: u8 = b'^';
static DOWN: u8 = b'v';
static LEFT: u8 = b'<';
static RIGHT: u8 = b'>';

#[derive(Clone, Debug)]
struct Pad {
    nexts: HashMap<u8, Vec<(u8, u8)>>,
    paths: HashMap<(u8, u8), HashSet<Vec<u8>>>,
}

impl Pad {
    fn new(map: HashMap<u8, Vec<(u8, u8)>>) -> Self {
        Self {
            nexts: map,
            paths: Default::default(),
        }
    }

    fn nexts(&self, from: u8) -> &Vec<(u8, u8)> {
        &self.nexts[&from]
    }

    fn shorts_from(&self, from: u8) -> HashMap<u8, HashSet<Vec<u8>>> {
        let mut done: HashMap<u8, HashSet<Vec<u8>>> = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((vec![], from));
        while let Some((path, node)) = q.pop_front() {
            let existing_len = done
                .get(&node)
                .and_then(|set| set.iter().next())
                .map(|one_path| one_path.len());
            if existing_len.is_none() || existing_len == Some(path.len()) {
                done.entry(node).or_default().insert(path.clone());
                q.extend(self.nexts(node).iter().map(|i| {
                    let mut newpath = path.clone();
                    newpath.push(i.0);
                    (newpath, i.1)
                }));
            }
        }
        done
    }
    fn add_cache(mut self) -> Self {
        let paths = self.nexts.keys().copied().fold(
            HashMap::new(),
            |mut hm: HashMap<(u8, u8), HashSet<Vec<u8>>>, from| {
                hm.extend(
                    self.shorts_from(from)
                        .into_iter()
                        .map(|(to, set)| ((from, to), set)),
                );
                hm
            },
        );
        self.paths = paths;
        self
    }
}

static NUMPAD: LazyLock<HashMap<u8, Vec<(u8, u8)>>> = LazyLock::new(|| {
    HashMap::from([
        (b'A', vec![(UP, b'3'), (LEFT, b'0')]),
        (b'0', vec![(UP, b'2'), (RIGHT, b'A')]),
        (b'1', vec![(UP, b'4'), (RIGHT, b'2')]),
        (b'2', vec![
            (UP, b'5'),
            (LEFT, b'1'),
            (DOWN, b'0'),
            (RIGHT, b'3'),
        ]),
        (b'3', vec![(UP, b'6'), (LEFT, b'2'), (DOWN, b'A')]),
        (b'4', vec![(UP, b'7'), (RIGHT, b'5'), (DOWN, b'1')]),
        (b'5', vec![
            (UP, b'8'),
            (LEFT, b'4'),
            (DOWN, b'2'),
            (RIGHT, b'6'),
        ]),
        (b'6', vec![(UP, b'9'), (LEFT, b'5'), (DOWN, b'3')]),
        (b'7', vec![(DOWN, b'4'), (RIGHT, b'8')]),
        (b'8', vec![(LEFT, b'7'), (DOWN, b'5'), (RIGHT, b'9')]),
        (b'9', vec![(LEFT, b'8'), (DOWN, b'6')]),
    ])
});
static DPAD: LazyLock<HashMap<u8, Vec<(u8, u8)>>> = LazyLock::new(|| {
    HashMap::from([
        (b'A', vec![(DOWN, b'>'), (LEFT, b'^')]),
        (b'^', vec![(RIGHT, b'A'), (DOWN, b'v')]),
        (b'v', vec![(UP, b'^'), (LEFT, b'<'), (RIGHT, b'>')]),
        (b'<', vec![(RIGHT, b'v')]),
        (b'>', vec![(UP, b'A'), (LEFT, b'v')]),
    ])
});

pub fn part1(input: &str) {
    let npaths = Pad::new(NUMPAD.clone()).add_cache().paths;
    let dpaths = Pad::new(DPAD.clone()).add_cache().paths;
    let costs: HashMap<(u8, u8), usize> = dpaths.keys().map(|pair| (*pair, 1)).collect();
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &npaths);
    let ans: usize = input
        .lines()
        .map(|l| {
            let mut alt = "A".to_string();
            alt.push_str(l);
            let pathlen = alt
                .bytes()
                .tuple_windows()
                .map(|(a, b)| costs[&(a, b)])
                .sum::<usize>();
            let numval: usize = l[0..3].parse().unwrap();
            numval * pathlen
        })
        .sum();
    answer(21, 1, ans);
}

fn layer(
    pair_costs: HashMap<(u8, u8), usize>,
    pair_paths: &HashMap<(u8, u8), HashSet<Vec<u8>>>, // pure movement paths
) -> HashMap<(u8, u8), usize> {
    pair_paths
        .iter()
        .map(|(pair, paths)| {
            let cost = paths
                .iter()
                .map(|path| {
                    let total_cost = if path.is_empty() {
                        1
                    } else {
                        let start_cost = pair_costs[&(b'A', path[0])];
                        let end_cost = pair_costs[&(path[path.len() - 1], b'A')];
                        let movement_cost: usize = path
                            .iter()
                            .tuple_windows()
                            .map(|(f, t)| pair_costs[&(*f, *t)])
                            .sum();
                        start_cost + end_cost + movement_cost
                    };
                    (total_cost, path)
                })
                .min()
                .unwrap()
                .0;
            (*pair, cost)
        })
        .collect()
}

pub fn part2(input: &str) {
    let npaths = Pad::new(NUMPAD.clone()).add_cache().paths;
    let dpaths = Pad::new(DPAD.clone()).add_cache().paths;
    let mut costs: HashMap<(u8, u8), usize> = dpaths.keys().map(|pair| (*pair, 1)).collect();
    for _ in 0..25 {
        costs = layer(costs, &dpaths);
    }
    let costs = layer(costs, &npaths);
    let ans: usize = input
        .lines()
        .map(|l| {
            let mut alt = "A".to_string();
            alt.push_str(l);
            let pathlen = alt
                .bytes()
                .tuple_windows()
                .map(|(a, b)| costs[&(a, b)])
                .sum::<usize>();
            let numval: usize = l[0..3].parse().unwrap();
            numval * pathlen
        })
        .sum();
    answer(21, 2, ans);
}
