use std::collections::HashMap;

use itertools::Itertools;

pub static SAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
pub static INPUT: &str = include_str!("../data/d01.txt");

pub fn part1(input: &str) {
    let mut a: Vec<isize> = Vec::new();
    let mut b: Vec<isize> = Vec::new();
    input.split_ascii_whitespace().tuples().for_each(|(x, y)| {
        a.push(x.parse().unwrap());
        b.push(y.parse().unwrap());
    });
    a.sort();
    b.sort();
    let day1_part1: isize = a.into_iter().zip(b).map(|(x, y)| (x - y).abs()).sum();
    dbg!(day1_part1);
}
pub fn part2(input: &str) {
    let mut a: Vec<isize> = Vec::new();
    let mut b: HashMap<isize, isize> = HashMap::new();
    input.split_ascii_whitespace().tuples().for_each(|(x, y)| {
        a.push(x.parse().unwrap());
        let e = b.entry(y.parse().unwrap());
        *e.or_default() += 1;
    });
    let ans: isize = a.into_iter().map(|x| x * b.get(&x).unwrap_or(&0)).sum();
    dbg!(ans);
}
