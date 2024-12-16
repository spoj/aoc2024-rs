use std::{
    collections::{BTreeSet, HashMap},
    iter,
};

use itertools::Itertools;

pub static SAMPLE: &str = r#"AA
AA"#;

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
    fn neis(&self, pos: usize) -> Vec<usize> {
        let current = self.data[pos];
        if current == 255 {
            vec![]
        } else {
            self.dirs()
                .into_iter()
                .map(|d| pos as isize + d)
                .filter(|next| self.in_bound(*next))
                .map(|x| x as usize)
                .collect_vec()
        }
    }
    fn recolor(&mut self) {
        let mut old = vec![99999; self.data.len()];
        std::mem::swap(&mut old, &mut self.data);
        let mut locs: BTreeSet<usize> = (0..old.len()).collect();
        let mut q: Vec<usize> = Vec::new();
        let mut n = 1;
        while let Some(i) = locs.pop_first() {
            q.push(i);
            let m = old[i];
            n += 1;
            while let Some(j) = q.pop() {
                locs.remove(&j);
                let neis = self.neis(j);
                neis.iter()
                    .filter(|k| old[**k] == m)
                    .filter(|k| self.data[**k] == 99999)
                    .for_each(|k| {
                        q.push(*k);
                    });

                self.data[j] = if old[j] == 0 { 0 } else { n };
            }
        }
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

    fn peri(&self) -> HashMap<usize, usize> {
        let mut out: HashMap<usize, usize> = HashMap::new();
        self.data.iter().tuple_windows().for_each(|(a, b)| {
            if a != b {
                *out.entry(*a).or_default() += 1;
                *out.entry(*b).or_default() += 1;
            }
        });

        (0..self.w)
            .flat_map(|i| (0..self.h).map(move |j| i + j * self.w))
            .map(|pos| self.data[pos as usize])
            .tuple_windows()
            .for_each(|(a, b)| {
                if a != b {
                    *out.entry(a).or_default() += 1;
                    *out.entry(b).or_default() += 1;
                }
            });
        out.remove(&0);
        out
    }
    fn pre_sides(&self) -> HashMap<usize, usize> {
        let mut out1: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut out2: HashMap<usize, Vec<usize>> = HashMap::new();
        self.data
            .iter()
            .tuple_windows()
            .enumerate()
            .for_each(|(i, (a, b))| {
                if a != b {
                    out1.entry(*a).or_default().push(self.transpose(i));
                    out1.entry(*b).or_default().push(self.transpose(i));
                }
            });

        (0..self.w)
            .flat_map(|i| (0..self.h).map(move |j| i + j * self.w))
            .map(|pos| self.data[pos as usize])
            .tuple_windows()
            .enumerate()
            .for_each(|(i, (a, b))| {
                if a != b {
                    out2.entry(a).or_default().push(i);
                    out2.entry(b).or_default().push(i);
                }
            });
        let mut output: HashMap<usize, usize> = HashMap::new();
        dbg!(&out1);
        dbg!(&out2);
        out1.into_iter()
            .for_each(|(a, b)| *output.entry(a).or_default() += discons(b));
        out2.into_iter()
            .for_each(|(a, b)| *output.entry(a).or_default() += discons(b));
        output.remove(&0);
        dbg!(&output);
        output
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
    fn prices(&self) -> HashMap<usize, usize> {
        let a = self.areas();
        let p = self.peri();
        let ans = a.iter().map(|(k, v)| (*k, p.get(k).unwrap() * v)).collect();
        ans
    }
    fn prices2(&self) -> HashMap<usize, usize> {
        let a = self.areas();
        let p = self.pre_sides();
        let ans = a
            .iter()
            .map(|(k, v)| {
                // dbg!((p.get(k).unwrap(), v));
                (*k, p.get(k).unwrap() * v)
            })
            .collect();
        ans
    }
}

pub fn part1(input: &str) {
    let mut b = Board::parse(input);
    b.recolor();
    dbg!(b.prices().values().sum::<usize>());
}

fn discons(mut data: Vec<usize>) -> usize {
    data.sort();
    let ans = data
        .iter()
        .tuple_windows()
        .filter(|(a, b)| **a + 1 != **b)
        .count()
        + 1;
    // println!("{:?} {}segs", &data, ans);
    ans
}

pub fn part2(input: &str) {
    let mut b = Board::parse(input);
    b.recolor();
    dbg!(b.prices2().values().sum::<usize>());
    todo!();
}
