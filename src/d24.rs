use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

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
struct ValueStore<'a> {
    kv: RefCell<HashMap<&'a str, isize>>,
    rels: HashMap<&'a str, (&'a str, &'a str, &'a str)>,
}

impl<'a> ValueStore<'a> {
    fn new(
        kv: HashMap<&'a str, isize>,
        rels: HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    ) -> Self {
        Self {
            kv: RefCell::new(kv),
            rels,
        }
    }
    fn resolve(&self, sym: &'a str) -> Option<isize> {
        if let Some(v) = self.kv.borrow().get(sym) {
            Some(*v)
        } else if let Some((op, a, b)) = self.rels.get(sym) {
            let a_val = self.resolve(a)?;
            let b_val = self.resolve(b)?;
            let new_val = match *op {
                "AND" => a_val & b_val,
                "OR" => a_val | b_val,
                "XOR" => a_val ^ b_val,
                _ => unreachable!(),
            };
            self.kv.borrow_mut().insert(sym, new_val);
            Some(new_val)
        } else {
            None
        }
    }
    fn all_nodes(&self) -> Vec<&str> {
        let mut out = HashSet::new();
        out.extend(self.kv.borrow().keys());
        out.extend(self.rels.keys());
        out.into_iter().collect()
    }
    fn znodes(&self) -> Vec<&str> {
        self.all_nodes()
            .into_iter()
            .filter(|x| x.starts_with('z'))
            .collect()
    }
    fn zvalues(&'a self) -> Vec<(isize, isize)> {
        self.znodes()
            .into_iter()
            .filter_map(|node| {
                let order: isize = node[1..3].parse().unwrap();
                let value = self.resolve(node)?;
                Some((order, value))
            })
            .collect()
    }
}

pub fn part1(input: &str) {
    let (a, b) = input.split_once("\n\n").unwrap();
    let init_values: HashMap<&str, isize> = a
        .lines()
        .map(|l| {
            let (nam, val) = l.split_once(": ").unwrap();
            (nam, val.parse().unwrap())
        })
        .collect();
    let re = Regex::new(r"([0-9a-z]+) ([A-Z]+) ([0-9a-z]+) -> ([0-9a-z]+)").unwrap();
    let rels: HashMap<&str, (&str, &str, &str)> = b
        .lines()
        .map(|l| {
            let (_, [a, b, c, d]) = re.captures(l).unwrap().extract();
            (d, (b, a, c))
        })
        .collect();
    let vs = ValueStore::new(init_values, rels);
    let ans: isize = vs
        .zvalues()
        .into_iter()
        .map(|(order, num)| num * 2isize.pow(order as u32))
        .sum();
    dbg!(ans);
}
