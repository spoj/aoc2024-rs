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

#[derive(Clone, Debug)]
struct Board {
    data: Vec<u8>,
    w: isize,
}

impl Board {
    fn new(input: Vec<Vec<u8>>) -> Self {
        let data = input.concat();
        let data: Vec<u8> = data.into_iter().collect();
        let w = input[0].len() as isize;
        Self { data, w }
    }

    fn parse(input: &str) -> Self {
        let mut input = input
            .lines()
            .map(|s| {
                iter::empty()
                    .chain(vec![WALL; s.bytes().len()])
                    .chain(s.bytes())
                    .chain(vec![WALL; s.bytes().len()])
                    .collect_vec()
            })
            .collect_vec();
        input.insert(0, vec![WALL; input[0].len()]);
        input.push(vec![WALL; input[0].len()]);
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

    fn start_to_end(&self) -> usize {
        self.dist_from_loc(self.find_start())[&self.find_end()]
    }
    fn dist_from_loc(&self, loc: isize) -> HashMap<isize, usize> {
        let mut done: HashMap<isize, usize> = HashMap::new();
        let mut h: BinaryHeap<(Reverse<usize>, isize)> = BinaryHeap::new();
        h.push((Reverse(0_usize), loc));
        while let Some((Reverse(dist), loc)) = h.pop() {
            if let Entry::Vacant(e) = done.entry(loc) {
                e.insert(dist);
                self.nexts(loc).for_each(|next| {
                    h.push((Reverse(1 + dist), next));
                });
            }
        }
        done
    }
    fn cheat_to(&self, pos: isize, radius: isize) -> Vec<(isize, usize)> {
        let x = pos % self.w;
        let y = pos / self.w;
        let mut out = Vec::new();
        for i in -radius..=radius {
            for j in -(radius - i.abs())..=(radius - i.abs()) {
                let newx = x + i;
                let newy = y + j;
                // println!("from {},{}, i is {} j is {} and to {},{}",x,y,i,j,newx,newy);
                let dist = (i.abs() + j.abs()) as usize;
                let new_pos = newy * self.w + newx;
                if dist > 1
                    && new_pos >= 0
                    && new_pos < self.data.len() as isize
                    && self.data[new_pos as usize] != WALL
                {
                    out.push((new_pos, dist));
                }
            }
        }
        out
    }
}

pub fn part1(thres: usize, input: &str) -> i32 {
    let board = Board::parse(input);
    let std_time = board.start_to_end();
    let from_start = board.dist_from_loc(board.find_start());
    let from_end = board.dist_from_loc(board.find_end());
    let mut ans = 0;
    for (a, leg1) in from_start {
        for (b, leg2) in board.cheat_to(a, 2) {
            if let Some(leg3) = from_end.get(&b) {
                if leg1 + leg2 + leg3 <= std_time - thres {
                    ans += 1;
                }
            }
        }
    }
    ans
}

pub fn part2(thres: usize, input: &str) -> i32 {
    let board = Board::parse(input);
    let std_time = board.start_to_end();
    let from_start = board.dist_from_loc(board.find_start());
    let from_end = board.dist_from_loc(board.find_end());
    let mut ans = 0;
    for (a, leg1) in from_start {
        for (b, leg2) in board.cheat_to(a, 20) {
            if let Some(leg3) = from_end.get(&b) {
                if leg1 + leg2 + leg3 <= std_time - thres {
                    ans += 1;
                }
            }
        }
    }
    ans
}
