use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, hash_map::Entry},
    iter,
};

use itertools::Itertools;

pub static SAMPLE: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;
pub static INPUT: &str = include_str!("../data/d20.txt");

static START: u8 = b'S';
static END: u8 = b'E';
static WALL: u8 = b'#';
static EMPTY: u8 = b'.';

#[derive(Clone, Debug)]
struct Board {
    data: Vec<u8>,
    w: isize,
    h: isize,
}

impl Board {
    fn new(input: Vec<Vec<u8>>) -> Self {
        let data = input.concat();
        let data: Vec<u8> = data.into_iter().collect();
        let w = input[0].len() as isize;
        let h = input.len() as isize;
        Self { data, w, h }
    }

    fn parse(input: &str) -> Self {
        let input = input
            .lines()
            .map(|s| {
                iter::empty()
                    // .chain(vec![0; s.bytes().len()])
                    .chain(s.bytes())
                    // .chain(vec![0; s.bytes().len()])
                    .collect_vec()
            })
            .collect_vec();
        // input.insert(0, vec![0; input[0].len()]);
        // input.push(vec![0; input[0].len()]);
        Board::new(input)
    }
    fn find_start(&self) -> isize {
        self.data.iter().position(|x| *x == START).unwrap() as isize
    }
    fn find_end(&self) -> isize {
        self.data.iter().position(|x| *x == END).unwrap() as isize
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
    fn is_end(&self, loc: isize) -> bool {
        self.data[loc as usize] == END
    }
    fn is_start(&self, loc: isize) -> bool {
        self.data[loc as usize] == START
    }
    fn corrupt_xy(&mut self, x: isize, y: isize) {
        let x = x + 1;
        let y = y + 1;
        let loc = y * self.w + x;
        self.data[loc as usize] = WALL;
    }

    fn passes(&self) -> bool {
        let end = self.find_end();
        let mut done: HashMap<isize, usize> = HashMap::new();
        let mut h: BinaryHeap<(Reverse<usize>, isize)> = BinaryHeap::new();
        h.push((Reverse(0_usize), self.find_start()));
        while let Some((Reverse(dist), loc)) = h.pop() {
            if let Entry::Vacant(e) = done.entry(loc) {
                e.insert(dist);
                self.nexts(loc).for_each(|next| {
                    h.push((Reverse(1 + dist), next));
                });
            }
        }
        done.contains_key(&end)
    }
    fn start_to_end(&self) -> usize {
        let end = self.find_end();
        let mut done: HashMap<isize, usize> = HashMap::new();
        let mut h: BinaryHeap<(Reverse<usize>, isize)> = BinaryHeap::new();
        h.push((Reverse(0_usize), self.find_start()));
        while let Some((Reverse(dist), loc)) = h.pop() {
            if let Entry::Vacant(e) = done.entry(loc) {
                e.insert(dist);
                self.nexts(loc).for_each(|next| {
                    h.push((Reverse(1 + dist), next));
                });
            }
        }
        done[&end]
    }
    fn is_bounds(&self, pos: isize) -> bool {
        let x = pos % self.w;
        let y = pos / self.h;
        x == 0 || x == self.w - 1 || y == 0 || y == self.h - 1
    }
    fn cheat_board(&self, pos: isize) -> Self {
        let mut c = self.clone();
        if !self.is_bounds(pos) && !self.is_start(pos) && !self.is_end(pos) {
            c.data[pos as usize] = b'.';
        }
        c
    }
}

pub fn part1(input: &str) {
    let b = Board::parse(input);
    let std_time = b.start_to_end() as isize;
    let x = (0..b.data.len())
        .map(|i| {
            let bb = b.cheat_board(i as isize);
            std_time - bb.start_to_end() as isize
        })
        .collect_vec(); kkk.,,yytt
    dbg!(x.iter().counts().iter().sorted().collect_vec());
}
pub fn part2(input: &str) {
    println!("{}", input);
}
