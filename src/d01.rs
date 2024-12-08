use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

pub static SAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

pub static INPUT: &str = include_str!("../data/d01.txt");

pub fn part1(input: &str) {
    let mut a: BinaryHeap<isize> = BinaryHeap::new();
    let mut b: BinaryHeap<isize> = BinaryHeap::new();
    input.lines().for_each(|x| {
        let vec = x
            .split_ascii_whitespace()
            .map(|num_as_str| num_as_str.parse().unwrap())
            .take(2)
            .collect_vec();
        a.push(vec[0]);
        b.push(vec[1]);
    });
    let day1_part1: isize = a.into_iter().zip(b).map(|(a, b)| (a - b).abs()).sum();
    dbg!(day1_part1);
}

pub fn part2(input: &str) {
    let mut left: Vec<isize> = vec![];
    let mut freq_r: HashMap<isize, isize> = HashMap::new();
    input.lines().for_each(|x| {
        let vec = x
            .split_ascii_whitespace()
            .map(|num_as_str| num_as_str.parse().unwrap())
            .take(2)
            .collect_vec();
        left.push(vec[0]);
        let right_num = vec[1];
        let e = freq_r.entry(right_num).or_default();
        *e += 1
    });

    let day1_part2: isize = left
        .iter()
        .map(|n| {
            let freq = freq_r.get(n).unwrap_or(&0);
            freq * n
        })
        .sum();
    dbg!(day1_part2);
}
