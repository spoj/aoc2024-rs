use std::iter;

use itertools::Itertools;

pub static SAMPLE0: &str = r#"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
"#;
pub static SAMPLE1: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;
pub static SAMPLE2: &str = r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
"#;

pub static INPUT: &str = include_str!("../data/d08.txt");

pub fn part1(input: &str) -> usize {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|s| {
            iter::empty()
                .chain(vec![b'#'; s.bytes().len()])
                .chain(s.bytes())
                .chain(vec![b'#'; s.bytes().len()])
                .collect_vec()
        })
        .collect_vec();
    let input_one = input.concat();
    let mut ans = vec![false; input_one.len()];
    for (i, a) in input_one.iter().enumerate() {
        for (j, b) in input_one.iter().enumerate() {
            let i = i as isize;
            let j = j as isize;
            let dif = j - i;
            if dif == 0 || *a == b'#' || *b == b'#' || *a == b'.' || *b == b'.' {
                continue;
            } else if a == b {
                let k = (j + dif) as usize;
                if input_one.get(k).is_some_and(|l| *l != b'#') {
                    ans[k] = true;
                }
            }
        }
    }
    let day8_part1 = ans.iter().filter(|x| **x).count();
    day8_part1
}

pub fn part2(input: &str) -> usize {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|s| {
            iter::empty()
                .chain(vec![b'#'; s.bytes().len()])
                .chain(s.bytes())
                .chain(vec![b'#'; s.bytes().len()])
                .collect_vec()
        })
        .collect_vec();
    let input_one = input.concat();
    let mut ans = vec![false; input_one.len()];
    for (i, a) in input_one.iter().enumerate() {
        for (j, b) in input_one.iter().enumerate() {
            let i = i as isize;
            let j = j as isize;
            let dif = j - i;
            if dif == 0 || *a == b'#' || *b == b'#' || *a == b'.' || *b == b'.' {
                continue;
            } else if a == b {
                let mut k = j;
                while input_one.get(k as usize).is_some_and(|l| *l != b'#') {
                    ans[k as usize] = true;
                    k += dif;
                }
            }
        }
    }
    let day8_part2 = ans.iter().filter(|x| **x).count();
    day8_part2
}
