use std::{collections::HashMap, iter::once};

use itertools::{Itertools, repeat_n};

pub static SAMPLE: &str = r#"029A
980A
179A
456A
379A
"#;
pub static INPUT: &str = include_str!("../data/d00.txt");

static UP: u8 = b'^';
static DOWN: u8 = b'v';
static LEFT: u8 = b'<';
static RIGHT: u8 = b'>';
static A: u8 = b'A';

fn dpad_pair(from: u8, to: u8) -> Vec<u8> {
    let locs: HashMap<u8, (isize, isize)> = HashMap::from([
        (b'7', (0, 0)),
        (b'8', (1, 0)),
        (b'9', (2, 0)),
        (b'4', (0, 1)),
        (b'5', (1, 1)),
        (b'6', (2, 1)),
        (b'1', (0, 2)),
        (b'2', (1, 2)),
        (b'3', (2, 2)),
        (b'0', (1, 3)),
        (b'A', (2, 3)),
    ]);
    let (from_x, from_y) = locs[&from];
    let (to_x, to_y) = locs[&to];
    let dist_y = to_y - from_y;
    let dist_x = to_x - from_x;
    let xchain = if dist_x > 0 {
        repeat_n(RIGHT, dist_x as usize)
    } else {
        repeat_n(LEFT, (-dist_x) as usize)
    };
    if dist_y > 0 {
        xchain.chain(repeat_n(DOWN, dist_y as usize))
    } else {
        repeat_n(UP, (-dist_y) as usize).chain(xchain)
    }
    .collect_vec()
}

fn shorts(dist_y: isize, dist_x: isize) -> Vec<Vec<u8>> {
    let xchain = if dist_x > 0 {
        repeat_n(RIGHT, dist_x as usize)
    } else {
        repeat_n(LEFT, (-dist_x) as usize)
    };
    let one = if dist_y > 0 {
        repeat_n(DOWN, dist_y as usize).chain(xchain)
    } else {
        xchain.chain(repeat_n(UP, (-dist_y) as usize))
    }
    .collect_vec();

    let many = one
        .iter()
        .copied()
        .permutations(one.len())
        .unique()
        .collect_vec();
    many
}

fn dir_pair(from: u8, to: u8) -> Vec<u8> {
    // print!("trying pair of {} and {}",from as char,to as char);
    let locs: HashMap<u8, (isize, isize)> = HashMap::from([
        (b'^', (1, 0)),
        (b'<', (0, 1)),
        (b'v', (1, 1)),
        (b'>', (2, 1)),
        (b'A', (2, 0)),
    ]);
    let (from_x, from_y) = locs[&from];
    let (to_x, to_y) = locs[&to];
    let dist_y = to_y - from_y;
    let dist_x = to_x - from_x;
    let xchain = if dist_x > 0 {
        repeat_n(RIGHT, dist_x as usize)
    } else {
        repeat_n(LEFT, (-dist_x) as usize)
    };
    if dist_y > 0 {
        repeat_n(DOWN, dist_y as usize).chain(xchain)
    } else {
        xchain.chain(repeat_n(UP, (-dist_y) as usize))
    }
    .collect_vec()
}

fn dpad_seq(seq: &[u8]) -> Vec<u8> {
    once(A)
        .chain(seq.iter().copied())
        .tuple_windows()
        .flat_map(|(a, b)| {
            let mut x = dpad_pair(a, b);
            x.push(A);
            x
        })
        .collect_vec()
}
fn dir_seq(seq: &[u8]) -> Vec<u8> {
    once(A)
        .chain(seq.iter().copied())
        .tuple_windows()
        .flat_map(|(a, b)| {
            let mut x = dir_pair(a, b);
            x.push(A);
            x
        })
        .collect_vec()
}

pub fn part1(input: &str) {
    let x: usize = input
        .lines()
        .map(|l| {
            let num: usize = l[0..3].parse().unwrap();
            let l = l.as_bytes();
            let len = dir_seq(&dir_seq(&dpad_seq(l))).len();
            println!("for {}, len is {}", num, len);
            println!(
                "seq {}",
                String::from_utf8(dir_seq(&dir_seq(&dpad_seq(l)))).unwrap()
            );
            println!("seq {}", String::from_utf8(dir_seq(&dpad_seq(l))).unwrap());
            println!("seq {}", String::from_utf8(dpad_seq(l)).unwrap());
            len * num
        })
        .sum();
    dbg!(x);
}

pub fn part2(input: &str) {
    println!("{}", input);
}
