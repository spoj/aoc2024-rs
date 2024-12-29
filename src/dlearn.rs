use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

static INPUT: &str = r"
1-2
2-3
3-6
6-9
9-8
8-7
7-4
";

pub fn trying() {
    let adj = INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.chars().next().unwrap(), b.chars().next().unwrap())
        })
        .collect_vec();
}
