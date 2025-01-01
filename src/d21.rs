use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque, hash_map::Entry},
    convert::identity,
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

static BOARD_STR: &str = r"
789
456
123
 0A";

static UP: u8 = b'^';
static DOWN: u8 = b'v';
static LEFT: u8 = b'<';
static RIGHT: u8 = b'>';
static A: u8 = b'A';
static SPACE: u8 = b' ';

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
        done.iter_mut().for_each(|(to, paths)| {
            paths.retain(|path| path.iter().chunk_by(|x| *x).into_iter().count() <= 2);
        });
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

    fn shorts_from_to(&self, from: u8, to: u8) -> &HashSet<Vec<u8>> {
        &self.paths[&(from, to)]
    }
    fn shorts_seq_int(&self, seq: &[u8]) -> HashSet<Vec<u8>> {
        if seq.is_empty() {
            HashSet::new()
        } else if seq.len() == 1 {
            self.shorts_from_to(b'A', seq[0]).clone()
        } else {
            let mut out = HashSet::new();
            let prev = self.shorts_seq_int(&seq[1..]);
            let head = self.shorts_from_to(seq[0], seq[1]);
            for i in prev {
                for j in head {
                    let combined = j
                        .clone()
                        .into_iter()
                        .chain(vec![b'A'])
                        .chain(i.clone())
                        .collect_vec();
                    out.insert(combined);
                }
            }
            out
        }
    }
    fn shorts_seq(&self, seq: &[u8]) -> HashSet<Vec<u8>> {
        let alt = [b'A'].into_iter().chain(seq.iter().copied()).collect_vec();
        self.shorts_seq_int(&alt)
    }
    fn short_merge<T>(&self, seqs: T) -> impl Iterator<Item = Vec<u8>>
    where
        T: IntoIterator<Item = Vec<u8>>,
    {
        let mut min_sofar = usize::MAX;
        let mut out = HashSet::new();
        seqs.into_iter().for_each(|seq| {
            let shorts = self.shorts_seq(&seq);
            let to_add = shorts.into_iter().filter(|short| {
                let len = short.len();
                min_sofar = min_sofar.min(len);
                short.len() <= min_sofar
            });
            out.extend(to_add);
        });
        out.into_iter().filter(move |x| x.len() <= min_sofar)
    }
}

fn simple_print_utf(data: &[u8]) {
    let string = String::from_utf8(data.to_owned()).to_owned().unwrap();
    println!("{}", string);
}

fn num_value(data: &str) -> usize {
    data[0..3].parse().unwrap()
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
    let n0 = Pad::new(NUMPAD.clone()).add_cache();
    let d1 = Pad::new(DPAD.clone()).add_cache();

    let ans = input
        .lines()
        .map(|l| {
            let num_value: usize = l[0..3].parse().unwrap();
            let shortest_path = d1
                .short_merge(d1.short_merge(n0.shorts_seq(l.as_bytes())))
                .next()
                .unwrap();
            shortest_path.len() * num_value
        })
        .sum::<usize>();
    answer(21, 2, ans);
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
    let costs: HashMap<(u8, u8), usize> = dpaths
        .iter()
        .map(|(pair, paths)| (*pair, paths.iter().next().unwrap().len() + 1))
        .collect();
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
    let costs = layer(costs, &dpaths);
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
    answer(21, 2, ans);
}
