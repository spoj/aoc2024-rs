use std::{
    collections::{BinaryHeap, HashMap, hash_map::Entry::Vacant},
    iter,
};

use itertools::Itertools;

pub static SAMPLE: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;
pub static INPUT: &str = include_str!("../data/d16.txt");
static UP: u8 = b'^';
static DOWN: u8 = b'v';
static LEFT: u8 = b'<';
static RIGHT: u8 = b'>';

static START: u8 = b'S';
static END: u8 = b'E';
static WALL: u8 = b'#';
#[derive(Clone, Debug)]
struct Board {
    data: Vec<u8>,
    w: isize,
    _h: isize,
}

impl Board {
    fn new(input: Vec<Vec<u8>>) -> Self {
        let data = input.concat();
        let data: Vec<u8> = data.into_iter().collect();
        let w = input[0].len() as isize;
        let h = input.len() as isize;
        Self { data, w, _h: h }
    }

    fn dir(&self, dir: u8) -> isize {
        match dir {
            d if d == UP => -self.w,
            d if d == DOWN => self.w,
            d if d == LEFT => -1,
            d if d == RIGHT => 1,
            _ => unreachable!(),
        }
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
    fn find_start(&self) -> (isize, u8) {
        (
            self.data.iter().position(|x| *x == START).unwrap() as isize,
            b'>',
        )
    }
    fn nexts(&self, pose: (isize, u8)) -> impl Iterator<Item = (isize, (isize, u8))> {
        [UP, DOWN, LEFT, RIGHT]
            .into_iter()
            .filter(move |mv| self.dir(*mv) != -self.dir(pose.1))
            .filter_map(move |mv| {
                let next = pose.0 + self.dir(mv);
                if self.data[next as usize] != WALL {
                    Some(if mv == pose.1 {
                        (-1, (next, mv))
                    } else {
                        (-1001, (next, mv))
                    })
                } else {
                    None
                }
            })
    }
    fn is_end(&self, loc: isize) -> bool {
        self.data[loc as usize] == END
    }
}

pub fn part1(input: &str) {
    let board = Board::parse(input);
    let mut h: BinaryHeap<(isize, (isize, u8))> = BinaryHeap::new();
    let mut done: HashMap<(isize, u8), isize> = HashMap::new();
    h.push((0, board.find_start()));
    while let Some((cost, pose)) = h.pop() {
        if let Vacant(e) = done.entry(pose) {
            e.insert(cost);
            board.nexts(pose).for_each(|(sub_cost, sub_pose)| {
                h.push((sub_cost + cost, sub_pose));
            });
        }
    }
    let day16_part1 = done
        .into_iter()
        .filter(|x| board.is_end(x.0.0))
        .map(|x| -x.1)
        .min()
        .unwrap();
    dbg!(day16_part1);
}
