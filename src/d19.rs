use std::{cell::RefCell, collections::HashMap};

use itertools::Itertools;

pub static SAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;
pub static INPUT: &str = include_str!("../data/d19.txt");

fn can_make(design: &str, pats: &[&str]) -> bool {
    // println!("entering can_make, design {}, pats {:?}", design, pats);
    if pats.is_empty() {
        false
    } else if design.is_empty() {
        true
    } else {
        pats.iter()
            .filter(|pat| design.starts_with(*pat))
            .any(|prefix| {
                let sub_design = &design[prefix.len()..];
                can_make(sub_design, pats)
            })
    }
}

struct PatsMemo {
    memo: RefCell<HashMap<String, usize>>,
    pats: Vec<String>,
}

impl PatsMemo {
    fn new(pats: &[&str]) -> Self {
        Self {
            memo: RefCell::new(HashMap::new()),
            pats: pats.iter().map(|x| x.to_string()).collect_vec(),
        }
    }

    fn ways2(&self, design: &str) -> usize {
        // println!("entering ways2, design {}", design);
        if let Some(ans) = self.memo.borrow().get(design) {
            *ans
        } else if self.pats.is_empty() {
            0
        } else if design.is_empty() {
            1
        } else {
            let ans = self
                .pats
                .iter()
                .filter(|pat| design.starts_with(*pat))
                .map(|prefix| {
                    let sub_design = &design[prefix.len()..];
                    self.ways2(sub_design)
                })
                .sum();
            self.memo.borrow_mut().insert(design.to_string(), ans);
            ans
        }
    }
}

pub fn part1(input: &str) {
    let (in1, in2) = input.split_once("\n\n").unwrap();
    let pats = in1.split(", ").collect_vec();
    let designs = in2.lines().collect_vec();
    let x = designs
        .into_iter()
        .filter(|design| can_make(design, &pats))
        .count();
    dbg!(x);
}
pub fn part2(input: &str) {
    let (in1, in2) = input.split_once("\n\n").unwrap();
    let pats = in1.split(", ").collect_vec();
    let designs = in2.lines().collect_vec();
    let pats_memo = PatsMemo::new(&pats);
    let x: usize = designs
        .into_iter()
        .map(|design| {
            // println!("=== {} ===", design);
            pats_memo.ways2(design)
        })
        .sum();
    dbg!(x);
}
