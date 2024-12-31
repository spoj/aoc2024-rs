use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashSet,
    iter,
};

use itertools::Itertools;

use crate::answer;

pub static SAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;
pub static INPUT: &str = include_str!("../data/d10.txt");

#[derive(Clone, Debug)]
struct Board {
    data: Vec<u8>,
    w: isize,
    h: isize,
}

impl Board {
    fn new(input: Vec<Vec<u8>>) -> Self {
        let data = input.concat();
        let w = input[0].len() as isize;
        let h = input.len() as isize;
        Self { data, w, h }
    }
    fn parse(input: &str) -> Self {
        let input = input
            .lines()
            .map(|s| {
                iter::empty()
                    .chain(vec![255; s.bytes().len()])
                    .chain(s.bytes().map(|x| x - b'0'))
                    .chain(vec![255; s.bytes().len()])
                    .collect_vec()
            })
            .collect_vec();
        Board::new(input)
    }

    fn in_bound(&self, pos: isize) -> bool {
        pos >= 0 && pos < self.w * self.h && self.data[pos as usize] != 255
    }
    fn dirs(&self) -> [isize; 4] {
        [-self.w, 1, self.w, -1]
    }
    fn nexts(&self, pos: isize) -> Vec<isize> {
        let current = self.data[pos as usize];
        if current == 255 {
            vec![]
        } else {
            self.dirs()
                .into_iter()
                .map(|d| pos + d)
                .filter(|next| self.in_bound(*next))
                .filter(|next| self.data[*next as usize] == current + 1)
                .collect_vec()
        }
    }
    fn starts(&self) -> impl Iterator<Item = isize> {
        (0..self.data.len())
            .filter(|pos| self.data[*pos] == 0)
            .map(|x| x as isize)
    }
    fn tails(&self, pos: isize) -> HashSet<isize> {
        match self.data[pos as usize].cmp(&9) {
            Greater => HashSet::new(),
            Equal => HashSet::from([pos]),
            Less => self
                .nexts(pos)
                .into_iter()
                .flat_map(|n| self.tails(n))
                .collect(),
        }
    }
    fn rating(&self, pos: isize) -> usize {
        match self.data[pos as usize].cmp(&9) {
            Greater => 0,
            Equal => 1,
            Less => self.nexts(pos).into_iter().map(|n| self.rating(n)).sum(),
        }
    }
}

pub fn part1(input: &str) {
    let input = Board::parse(input);
    let day10_part1: usize = input.starts().map(|start| input.tails(start).len()).sum();
    answer(10, 1, day10_part1);
}

pub fn part2(input: &str) {
    let input = Board::parse(input);
    let day10_part2: usize = input.starts().map(|start| input.rating(start)).sum();
    answer(10, 2, day10_part2);
}
