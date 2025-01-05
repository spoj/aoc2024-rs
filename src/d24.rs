use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;
use rand::{Rng, rngs::ThreadRng, thread_rng};
use regex::Regex;

pub static SAMPLE: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#;
pub static INPUT: &str = include_str!("../data/d24.txt");

#[derive(Debug, PartialEq, Eq)]
struct ValueStore {
    kv: HashMap<String, usize>,
    rels: HashMap<String, (String, String, String)>,
}

impl ValueStore {
    fn new(kv: HashMap<String, usize>, rels: HashMap<String, (String, String, String)>) -> Self {
        Self { kv, rels }
    }

    fn ff(&mut self) {
        let mut queue: Vec<_> = self.rels.keys().collect();
        let mut progress = true;
        while progress {
            progress = false;
            queue.retain(|sym| {
                let (op, a, b) = self.rels.get(&sym[..]).unwrap();
                if self.kv.contains_key(a) && self.kv.contains_key(b) {
                    let a_val = self.kv[a];
                    let b_val = self.kv[b];
                    progress = true;
                    let new_val = match &op[..] {
                        "AND" => a_val & b_val,
                        "OR" => a_val | b_val,
                        "XOR" => a_val ^ b_val,
                        _ => unreachable!(),
                    };
                    self.kv.insert(sym.to_string(), new_val);
                    false
                } else {
                    true
                }
            });
        }
    }
    fn znodes(&self) -> Vec<&str> {
        self.rels
            .keys()
            .filter(|x| x.starts_with('z'))
            .map(|x| &x[..])
            .collect()
    }

    fn zvalue(&mut self) -> usize {
        self.ff();
        let mut sum = 0;
        for node in self.znodes() {
            let order: usize = node[1..3].parse().unwrap();
            let value = self.kv.get(node).unwrap_or(&0);
            sum += value * 2usize.pow(order as u32);
        }
        sum
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AddingMachine {
    rels: HashMap<String, (String, String, String)>,
    nodes: Vec<String>,
    likely: HashSet<(String, String)>,
}

static STEP: usize = 8;
static TESTS: usize = 64;
impl AddingMachine {
    fn new(rels: HashMap<String, (String, String, String)>) -> Self {
        let nodes = rels.keys().cloned().sorted().collect_vec();
        Self {
            rels,
            nodes,
            likely: Default::default(),
        }
    }
    fn to_bits(mut num: usize) -> Vec<usize> {
        iter::from_fn(|| {
            let bit = num % 2;
            num >>= 1;
            Some(bit)
        })
        .take(45)
        .collect()
    }

    fn add(&self, x: usize, y: usize) -> usize {
        let mut kv = HashMap::new();
        kv.extend(
            Self::to_bits(x)
                .into_iter()
                .enumerate()
                .map(|(pos, v)| (format!("x{:02}", pos), v)),
        );
        kv.extend(
            Self::to_bits(y)
                .into_iter()
                .enumerate()
                .map(|(pos, v)| (format!("y{:02}", pos), v)),
        );
        let mut vs = ValueStore::new(kv, self.rels.clone());
        vs.ff();
        vs.zvalue()
    }
    fn coincident(&self, x: usize, y: usize) -> Option<HashMap<String, usize>> {
        let mut kv = HashMap::new();
        kv.extend(
            Self::to_bits(x)
                .into_iter()
                .enumerate()
                .map(|(pos, v)| (format!("x{:02}", pos), v)),
        );
        kv.extend(
            Self::to_bits(y)
                .into_iter()
                .enumerate()
                .map(|(pos, v)| (format!("y{:02}", pos), v)),
        );
        let mut vs = ValueStore::new(kv, self.rels.clone());
        vs.ff();
        if vs.zvalue() == (x + y) {
            Some(vs.kv)
        } else {
            None
        }
    }
    fn many_coincidents(&self, tries: usize, rng: &mut ThreadRng) -> HashMap<String, Vec<usize>> {
        let mut out = HashMap::new();
        let mut cnt = 0;
        let bits = 44;
        while cnt < tries {
            let x: usize = rng.gen_range(0..1 << bits);
            let y: usize = rng.gen_range(0..1 << bits);
            if let Some(m) = self.coincident(x, y) {
                for (k, v) in m {
                    let e: &mut Vec<usize> = out.entry(k).or_default();
                    e.push(v);
                }
                cnt += 1;
            }
        }
        out
    }
    fn swap(&mut self, left: &str, right: &str) {
        if left == right {
            unreachable!();
        }
        let left_value = self.rels.remove(left).unwrap();
        let right_value = self.rels.remove(right).unwrap();
        self.rels.insert(left.to_string(), right_value);
        self.rels.insert(right.to_string(), left_value);
    }
    fn ver(&self, bits: usize) -> bool {
        let mut rng = thread_rng();
        for _ in 0..TESTS {
            let x = rng.gen_range(0..1 << bits);
            let y = rng.gen_range(0..1 << bits);
            if self.add(x, y) != x + y {
                return false;
            }
        }
        true
    }

    fn solve2(
        &mut self,
        bits: usize,
        mut swaps: Vec<(String, String)>,
        rng: &mut ThreadRng,
    ) -> Option<Vec<(String, String)>> {
        let verified = self.ver(bits);
        if bits >= 45 {
            Some(swaps)
        } else if swaps.len() >= 4 && !verified {
            None
        } else if verified {
            self.solve2(bits + 1, swaps.clone(), rng)
        } else {
            let nodes = self.nodes.clone();
            let rest = nodes
                .clone()
                .into_iter()
                .cartesian_product(nodes.clone())
                .filter(|(a, b)| (a < b) && !(a.starts_with('z') && a.starts_with('z')));
            self.likely_pairs(rng);
            let likely = self.likely.clone();
            let all = likely.into_iter().chain(rest);
            for (i, j) in all {
                self.swap(&i, &j);
                swaps.push((i.to_string(), j.to_string()));
                if swaps.len() <= 4 && self.ver(bits) {
                    if let Some(x) = self.solve2(bits + STEP, swaps.clone(), rng) {
                        return Some(x);
                    }
                }
                self.swap(&i, &j);
                swaps.pop();
            }
            None
        }
    }
    fn likely_pairs(&mut self, rng: &mut ThreadRng) {
        let coincides = self.many_coincidents(32, rng);
        for (k1, v1) in &coincides {
            for (k2, v2) in &coincides {
                if k1 != k2
                    && v1 == v2
                    && !v1.iter().all(|x| *x == 0)
                    && !k1.starts_with('x')
                    && !k1.starts_with('y')
                    && !k2.starts_with('x')
                    && !k2.starts_with('y')
                {
                    if k1 < k2 {
                        self.likely.insert((k1.to_string(), k2.to_string()));
                    } else {
                        self.likely.insert((k2.to_string(), k1.to_string()));
                    }
                }
            }
        }
    }
}
pub fn part1(input: &str) -> usize {
    let (a, b) = input.split_once("\n\n").unwrap();
    let init_values: HashMap<String, usize> = a
        .lines()
        .map(|l| {
            let (nam, val) = l.split_once(": ").unwrap();
            (nam.to_string(), val.parse().unwrap())
        })
        .collect();
    let re = Regex::new(r"([0-9a-z]+) ([A-Z]+) ([0-9a-z]+) -> ([0-9a-z]+)").unwrap();
    let rels: HashMap<String, (String, String, String)> = b
        .lines()
        .map(|l| {
            let (_, [a, b, c, d]) = re.captures(l).unwrap().extract();
            (d.to_string(), (b.to_string(), a.to_string(), c.to_string()))
        })
        .collect();
    let mut vs = ValueStore::new(init_values, rels);
    let day24_part1: usize = vs.zvalue();
    day24_part1
}

pub fn part2(input: &str) -> String {
    let (_, b) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r"([0-9a-z]+) ([A-Z]+) ([0-9a-z]+) -> ([0-9a-z]+)").unwrap();
    let rels: HashMap<String, (String, String, String)> = b
        .lines()
        .map(|l| {
            let (_, [a, b, c, d]) = re.captures(l).unwrap().extract();
            (d.to_string(), (b.to_string(), a.to_string(), c.to_string()))
        })
        .collect();
    let adder = AddingMachine::new(rels);
    let mut solver = adder.clone();
    let mut rng = thread_rng();
    let ans = solver
        .solve2(STEP, vec![], &mut rng)
        .unwrap()
        .into_iter()
        .flat_map(|x| [x.0, x.1])
        .sorted()
        .collect_vec();
    format!("{}", ans.into_iter().format(","))
}
