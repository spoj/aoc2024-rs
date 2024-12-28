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

pub fn part1(input: &str) {
    dbg!(input);
}

pub fn part2(input: &str) {
    dbg!(input);
}
