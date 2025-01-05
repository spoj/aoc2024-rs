use std::{
    collections::{HashMap, hash_map},
    iter::successors,
};

use itertools::Itertools;

pub static SAMPLE: &str = r#"1
2
3
2024
"#;
pub static INPUT: &str = include_str!("../data/d22.txt");

fn nextsec(from: isize) -> isize {
    let mix = |old, i| old ^ i;
    let prune = |i| i % 16777216;

    let a = prune(mix(from, from * 64));
    let b = prune(mix(a, a / 32));
    prune(mix(b, b * 2048))
}

fn gens(from: isize, steps: isize) -> isize {
    std::iter::successors(Some(from), |&x| Some(nextsec(x)))
        .nth(steps as usize)
        .unwrap()
}

pub fn part1(input: &str) -> isize {
    let ans: isize = input.lines().map(|l| gens(l.parse().unwrap(), 2000)).sum();
    ans
}
pub fn part2(input: &str) -> isize {
    let inits: Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect_vec();
    let sig_to_banana: Vec<HashMap<[isize; 4], isize>> = inits
        .iter()
        .map(|init| {
            successors(Some(*init), |x| Some(nextsec(*x)))
                .take(2000)
                .tuple_windows()
                .map(|(a, b, c, d, e)| {
                    let diffs = [
                        (b % 10 - a % 10),
                        (c % 10 - b % 10),
                        (d % 10 - c % 10),
                        (e % 10 - d % 10),
                    ];
                    let val = e % 10;
                    (diffs, val)
                })
                .fold(HashMap::new(), |mut a, (k, v)| {
                    if let hash_map::Entry::Vacant(e) = a.entry(k) {
                        e.insert(v);
                    }
                    a
                })
        })
        .collect_vec();

    let out = sig_to_banana
        .iter()
        .fold(HashMap::new(), |mut a: HashMap<[isize; 4], isize>, b| {
            for (k, v) in b {
                let e = a.entry(*k).or_default();
                *e += v;
            }
            a
        });
    *out.iter().map(|(a, b)| (b, a)).max().unwrap().0
}
