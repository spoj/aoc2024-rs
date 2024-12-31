use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, hash_map::Entry::Vacant},
    iter,
};

use itertools::Itertools;

use crate::answer;

pub static SAMPLE: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;
pub static INPUT: &str = include_str!("../data/d18.txt");
#[derive(Clone, Debug)]
struct Board {
    data: Vec<u8>,
    w: isize,
    h: isize,
}

static WALL: u8 = b'#';
static EMPTY: u8 = b'.';

impl Board {
    fn new(w: isize, h: isize) -> Self {
        let body = (0..h).flat_map(|_| {
            iter::empty()
                .chain(vec![WALL; 1])
                .chain(vec![EMPTY; w as usize])
                .chain(vec![WALL; 1])
                .collect_vec()
        });
        let data = iter::empty()
            .chain(vec![WALL; w as usize + 2])
            .chain(body)
            .chain(vec![WALL; w as usize + 2])
            .collect_vec();
        Self {
            data,
            w: w + 2,
            h: h + 2,
        }
    }
    fn end(&self) -> isize {
        let x = self.h - 2;
        let y = self.h - 2;
        y * self.w + x
    }
    fn start(&self) -> isize {
        let x = 1;
        let y = 1;
        y * self.w + x
    }
    fn nexts(&self, pos: isize) -> impl Iterator<Item = isize> {
        [-self.w, self.w, -1, 1].into_iter().filter_map(move |mv| {
            let next = pos + mv;
            if self.data[next as usize] != WALL {
                Some(next)
            } else {
                None
            }
        })
    }
    fn corrupt_xy(&mut self, x: isize, y: isize) {
        let x = x + 1;
        let y = y + 1;
        let loc = y * self.w + x;
        self.data[loc as usize] = WALL;
    }

    fn passes(&self) -> bool {
        let mut done: HashMap<isize, usize> = HashMap::new();
        let mut h: BinaryHeap<(Reverse<usize>, isize)> = BinaryHeap::new();
        h.push((Reverse(0_usize), self.start()));
        while let Some((Reverse(dist), loc)) = h.pop() {
            if let Vacant(e) = done.entry(loc) {
                e.insert(dist);
                self.nexts(loc).for_each(|next| {
                    h.push((Reverse(1 + dist), next));
                });
            }
        }
        done.contains_key(&self.end())
    }
}

fn passes_after(w: isize, h: isize, bytes: &[(isize, isize)]) -> bool {
    let mut b = Board::new(w, h);
    for (x, y) in bytes {
        b.corrupt_xy(*x, *y);
    }
    b.passes()
}

pub fn part1(w: isize, h: isize, len: usize, input: &str) {
    let mut board = Board::new(w, h);
    dbg!(board.end());
    input.lines().take(len).for_each(|l| {
        let (x, y) = l.split_once(',').unwrap();
        board.corrupt_xy(x.parse().unwrap(), y.parse().unwrap());
        // board.pretty();
    });
    let mut done: HashMap<isize, usize> = HashMap::new();
    let mut h: BinaryHeap<(Reverse<usize>, isize)> = BinaryHeap::new();
    h.push((Reverse(0_usize), board.start()));
    while let Some((Reverse(dist), loc)) = h.pop() {
        if let Vacant(e) = done.entry(loc) {
            e.insert(dist);
            board.nexts(loc).for_each(|next| {
                h.push((Reverse(1 + dist), next));
            });
        }
    }
    // board.pretty();
    answer(18, 1, done[&board.end()]);
}

pub fn part2(w: isize, h: isize, input: &str) {
    let bytes: Vec<(isize, isize)> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect_vec();
    let mut a = 0;
    let mut b = bytes.len() - 1;
    while b - a > 1 {
        let c = (b - a) / 2 + a;
        if passes_after(w, h, &bytes[0..c]) {
            a = c;
        } else {
            b = c
        }
    }
    let day18_part2 = bytes[a];
    answer(18, 2, format!("{:?}", day18_part2));
}
