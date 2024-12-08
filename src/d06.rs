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

pub fn part1(input: &str) {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|s| {
            iter::empty()
                .chain(*b"x")
                .chain(s.bytes())
                .chain(*b"x")
                .collect_vec()
        })
        .collect_vec();
    let w = input[0].len();
    let h = input.len();
    let ww = w as isize;
    let hh = h as isize;
    let input_one = input.concat();
    let mut visited = vec![false; (ww * hh) as usize];
    let dirs = [-ww, 1, ww, -1];
    let mut cursor = input_one.iter().position(|x| *x == b'^').unwrap() as isize;
    'a: for dir in dirs.iter().cycle() {
        // dbg!((cursor % ww, cursor / ww));
        'b: loop {
            if cursor < 0 || cursor >= ww * hh || input_one[(cursor) as usize] == b'x' {
                break 'a;
            } else if input_one.get((cursor + *dir) as usize) == Some(&b'#') {
                visited[cursor as usize] = true;
                break 'b;
            } else {
                visited[cursor as usize] = true;
                cursor += dir;
            }
        }
    }
    let day6_part1 = visited.iter().filter(|x| **x).count();
    dbg!(day6_part1);
}

pub fn part2(input: &str) {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|s| {
            iter::empty()
                .chain(*b"x")
                .chain(s.bytes())
                .chain(*b"x")
                .collect_vec()
        })
        .collect_vec();
    let w = input[0].len();
    let h = input.len();
    let ww = input[0].len() as isize;
    let hh = input.len() as isize;
    let mut input_one = input.concat();
    let mut day6_part2 = 0;
    for i in 0..w * h {
        let t = input_one[i];
        input_one[i] = b'#';
        if t != b'x' && have_loop(&input_one, ww, hh) {
            day6_part2 += 1;
        }
        input_one[i] = t;
    }
    dbg!(day6_part2);
}

pub fn have_loop(input_one: &[u8], ww: isize, hh: isize) -> bool {
    let mut obstructed = vec![0; (ww * hh) as usize];
    let dirs = [-ww, 1, ww, -1];
    let Some(cursor) = input_one.iter().position(|x| *x == b'^') else {
        return false;
    };
    let mut cursor = cursor as isize;
    'a: for dir in dirs.iter().cycle() {
        'b: loop {
            if cursor < 0 || cursor >= ww * hh || input_one[(cursor) as usize] == b'x' {
                break 'a;
            } else if input_one.get((cursor + *dir) as usize) == Some(&b'#') {
                if obstructed[cursor as usize] > 1 {
                    return true;
                }
                obstructed[cursor as usize] += 1;
                break 'b;
            } else {
                cursor += dir;
            }
        }
    }
    false
}
