use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    iter,
};

use itertools::Itertools;

use crate::answer;

pub static SAMPLE: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

pub static INPUT: &str = include_str!("../data/d12.txt");

#[derive(Clone, Debug)]
struct Board {
    data: Vec<usize>,
    w: isize,
    h: isize,
}

impl Board {
    fn new(input: Vec<Vec<u8>>) -> Self {
        let data = input.concat();
        let data = data.into_iter().map(|x| x as usize).collect();
        let w = input[0].len() as isize;
        let h = input.len() as isize;
        Self { data, w, h }
    }

    fn in_bound(&self, pos: isize) -> bool {
        pos >= 0 && pos < self.w * self.h && self.data[pos as usize] != 255
    }
    fn dirs(&self) -> [isize; 4] {
        [-self.w, 1, self.w, -1]
    }
    fn neis(&self, pos: isize) -> Vec<isize> {
        let current = self.data[pos as usize];
        if current == 255 {
            vec![]
        } else {
            self.dirs()
                .into_iter()
                .map(|d| pos + d)
                .filter(|next| self.in_bound(*next))
                .collect_vec()
        }
    }
    fn circle(&self, pos: isize) -> Vec<(isize, (isize, isize))> {
        vec![
            (pos, (1, 0)),
            (pos, (0, -1)),
            (pos + 1, (0, 1)),
            (pos + self.w, (-1, 0)),
        ]
    }
    fn dual_circles(&self, pos: isize, (x, y): (isize, isize)) -> Vec<(isize, (isize, isize))> {
        vec![
            (pos + 1, (-x, 0)),
            (pos, (x, 0)),
            (pos +  self.w, (0, -y)),
            (pos, (0, y)),
        ]
    }
    fn regions(&self) -> Vec<BTreeSet<isize>> {
        let mut out = Vec::new();
        let mut locations: BTreeSet<isize> = (0..self.data.len()).map(|x| x as isize).collect();
        while let Some(init) = locations.pop_first() {
            let letter = self.data[init as usize];
            if letter == 0 {
                continue;
            }
            let mut reached: BTreeSet<isize> = BTreeSet::new();
            let mut stack = vec![init];
            while let Some(node) = stack.pop() {
                if !reached.contains(&node) {
                    reached.insert(node);
                    locations.remove(&node);
                    stack.extend(
                        self.neis(node)
                            .iter()
                            .filter(|loc| self.data[**loc as usize] == letter),
                    );
                }
            }
            out.push(reached);
        }

        out
    }

    fn parse(input: &str) -> Self {
        let mut input = input
            .lines()
            .map(|s| {
                iter::empty()
                    .chain(vec![0; s.bytes().len()])
                    .chain(s.bytes())
                    .chain(vec![0; s.bytes().len()])
                    .collect_vec()
            })
            .collect_vec();
        input.insert(0, vec![0; input[0].len()]);
        input.push(vec![0; input[0].len()]);
        Board::new(input)
    }

    fn transpose(&self, loc: usize) -> usize {
        let x = loc % self.w as usize;
        let y = loc / self.w as usize;
        x * self.h as usize + y
    }
    fn areas(&self) -> HashMap<usize, usize> {
        let mut out: HashMap<usize, usize> = HashMap::new();
        self.data.iter().for_each(|a| {
            *out.entry(*a).or_default() += 1;
        });
        out.remove(&0);
        out
    }
}

pub fn part1(input: &str) {
    let b = Board::parse(input);
    let x: usize = b
        .regions()
        .into_iter()
        .map(|reg| {
            let area = reg.len();
            let circles = reg.iter().flat_map(|loc| b.circle(*loc)).fold(
                BTreeMap::new(),
                |mut out: BTreeMap<isize, (isize, isize)>, (at, vec)| {
                    let e = out.entry(at).or_default();
                    e.0 += vec.0;
                    e.1 += vec.1;
                    out
                },
            );
            let peri: usize = circles
                .values()
                .map(|(a, b)| (a.abs() + b.abs()) as usize)
                .sum();
            peri * area
        })
        .sum();
    answer(12, 1, x);
}

pub fn part2(input: &str) {
    let b = Board::parse(input);
    let x: usize = b
        .regions()
        .into_iter()
        .map(|reg| {
            let area = reg.len();
            let circles = reg.iter().flat_map(|loc| b.circle(*loc)).fold(
                BTreeMap::new(),
                |mut out: BTreeMap<isize, (isize, isize)>, (at, vec)| {
                    let e = out.entry(at).or_default();
                    e.0 += vec.0;
                    e.1 += vec.1;
                    out
                },
            );
            let diffed = circles
                .into_iter()
                .flat_map(|(pos, (x, y))| b.dual_circles(pos, (x, y)))
                .fold(
                    BTreeMap::new(),
                    |mut out: BTreeMap<isize, (isize, isize)>, (at, vec)| {
                        let e = out.entry(at).or_default();
                        e.0 += vec.0;
                        e.1 += vec.1;
                        out
                    },
                );
            let sides: usize = diffed
                .values()
                .map(|(a, b)| (a.abs() + b.abs()) as usize)
                .sum::<usize>()
                / 2;
            sides * area
        })
        .sum();
    answer(12, 2, x);
}
