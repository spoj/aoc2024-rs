use std::iter;

use itertools::Itertools;

pub static SAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

pub static SAMPLE2: &str = r#"....#.....
....+---+#
....|...|.
..#.|...|.
....|..#|.
....|...|.
.#.#^---+.
........#.
#.........
......#...
"#;

pub static INPUT: &str = include_str!("../data/d06.txt");

#[derive(Clone)]
struct Board {
    data: Vec<u8>,
    visited: Vec<bool>,
    w: isize,
    h: isize,
}

impl Board {
    fn new(input: Vec<Vec<u8>>) -> Self {
        let data = input.concat();
        let w = input[0].len() as isize;
        let h = input.len() as isize;
        let visited = vec![false; data.len()];
        Self {
            data,
            visited,
            w,
            h,
        }
    }
    fn parse(input: &str) -> Self {
        let input = input
            .lines()
            .map(|s| {
                iter::empty()
                    .chain(vec![b'x'; s.bytes().len()])
                    .chain(s.bytes())
                    .chain(vec![b'x'; s.bytes().len()])
                    .collect_vec()
            })
            .collect_vec();
        Board::new(input)
    }
    fn turn(&self, dir: isize) -> isize {
        if dir == -self.w {
            1
        } else if dir == 1 {
            self.w
        } else if dir == self.w {
            -1
        } else {
            -self.w
        }
    }
    fn mark_visit(&mut self, pos: isize) {
        self.visited[pos as usize] = true
    }
    fn count_visited(&self) -> usize {
        self.visited.iter().filter(|x| **x).count()
    }
    fn in_bound(&self, pos: isize) -> bool {
        pos >= 0 && pos < self.w * self.h && self.data[pos as usize] != b'x'
    }
}

pub fn part1(input: &str) -> usize {
    let mut board = Board::parse(input);
    let mut pos = board.data.iter().position(|x| *x == b'^').unwrap() as isize;
    let mut dir = board.turn(0);
    while board.in_bound(pos) {
        board.mark_visit(pos);
        if board.data.get((pos + dir) as usize) == Some(&b'#') {
            dir = board.turn(dir);
        } else {
            pos += dir;
        }
    }
    board.count_visited()
}
pub fn visited(input: &str) -> Vec<isize> {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|s| {
            iter::empty()
                .chain(vec![b'x'; s.bytes().len()])
                .chain(s.bytes())
                .chain(vec![b'x'; s.bytes().len()])
                .collect_vec()
        })
        .collect_vec();
    let mut board = Board::new(input);
    let mut pos = board.data.iter().position(|x| *x == b'^').unwrap() as isize;
    let mut dir = board.turn(0);
    while board.in_bound(pos) {
        board.mark_visit(pos);
        if board.data.get((pos + dir) as usize) == Some(&b'#') {
            dir = board.turn(dir);
        } else {
            pos += dir;
        }
    }
    board
        .visited
        .iter()
        .enumerate()
        .filter(|(_a, b)| **b)
        .map(|(a, _b)| a as isize)
        .collect_vec()
}

pub fn part2(input: &str) -> i32 {
    let visited = visited(input);
    let mut board = Board::parse(input);
    let mut day6_part2 = 0;
    for i in visited {
        let t = board.data[i as usize];
        board.data[i as usize] = b'#';
        if t != b'x' && have_loop(&board) {
            day6_part2 += 1;
        }
        board.data[i as usize] = t;
    }
    day6_part2
}

fn have_loop(board: &Board) -> bool {
    let mut obstructed = vec![0; (board.w * board.h) as usize];
    let Some(pos) = board.data.iter().position(|x| *x == b'^') else {
        return false;
    };
    let mut pos = pos as isize;
    let mut dir = board.turn(0);
    while board.in_bound(pos) {
        if board.data.get((pos + dir) as usize) == Some(&b'#') {
            if obstructed[pos as usize] > 1 {
                return true;
            }
            obstructed[pos as usize] += 1;
            dir = board.turn(dir);
        } else {
            pos += dir;
        }
    }
    false
}
