use std::collections::HashMap;

use crate::answer;

pub static SAMPLE: &str = r#"125 17"#;
pub static INPUT: &str = include_str!("../data/d11.txt");

pub fn part1(input: &str) {
    let mut a: HashMap<usize, usize> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .for_each(|x| {
            a.insert(x, 1);
        });
    for _ in 0..25 {
        let mut b: HashMap<usize, usize> = HashMap::new();
        for (n, dup) in &a {
            for next in next_nums(*n) {
                let e = b.entry(next).or_default();
                *e += dup;
            }
        }
        a = b;
    }
    let ans: usize = a.values().sum();
    answer(11, 1, ans);
}
pub fn part2(input: &str) {
    let mut a: HashMap<usize, usize> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .for_each(|x| {
            a.insert(x, 1);
        });
    for _ in 0..75 {
        let mut b: HashMap<usize, usize> = HashMap::new();
        for (n, dup) in &a {
            for next in next_nums(*n) {
                let e = b.entry(next).or_default();
                *e += dup;
            }
        }
        a = b;
    }
    let day11_part2: usize = a.values().sum();
    answer(11, 2, day11_part2);
}

fn digits(num: usize) -> usize {
    if num == 0 {
        0
    } else {
        std::iter::successors(Some(num), {
            |x| if *x < 10 { None } else { Some(*x / 10) }
        })
        .count()
    }
}

fn next_nums(num: usize) -> Vec<usize> {
    // println!("check next for {}",num);
    let dig = digits(num);
    match dig {
        0 => {
            vec![1]
        }
        _ if dig % 2 == 0 => {
            let pow = 10usize.pow(dig as u32 / 2);
            let left = num / pow;
            let right = num - left * pow;
            vec![left, right]
        }
        _ => {
            vec![num * 2024]
        }
    }
}
