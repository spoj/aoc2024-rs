use std::io::stdin;

use itertools::Itertools;
use regex::Regex;

use crate::answer;

pub static SAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;
pub static INPUT: &str = include_str!("../data/d14.txt");

pub fn part1(w: isize, h: isize, secs: isize, input: &str) {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let bots: Vec<[isize; 4]> = input
        .lines()
        .map(|bot_str| {
            let (_a, b): (_, [&str; 4]) = re.captures(bot_str).unwrap().extract();
            b.map(|numstr| numstr.parse().unwrap())
        })
        .collect_vec();
    let final_locs = bots
        .iter()
        .map(|[a, b, c, d]| (cmod(a + c * secs, w), cmod(b + d * secs, h)))
        .collect_vec();
    let quads = final_locs
        .iter()
        .filter(|(a, b)| (*a != ((w - 1) / 2)) && (*b != (h - 1) / 2))
        .map(|(a, b)| (*a < w / 2, *b < h / 2))
        .counts();
    let ans: usize = quads.values().product();

    answer(14, 1, ans);
}

pub fn sim_sec(w: isize, h: isize, secs: isize, input: &str) {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let bots: Vec<[isize; 4]> = input
        .lines()
        .map(|bot_str| {
            let (_a, b): (_, [&str; 4]) = re.captures(bot_str).unwrap().extract();
            b.map(|numstr| numstr.parse().unwrap())
        })
        .collect_vec();
    let final_locs = bots
        .iter()
        .map(|[a, b, c, d]| (cmod(a + c * secs, w), cmod(b + d * secs, h)))
        .collect_vec();
    let mut output = vec![vec![b'.'; w as usize]; h as usize];
    final_locs
        .iter()
        .for_each(|(a, b)| output[*b as usize][*a as usize] = b'#');
    for line in output {
        for c in line {
            print!("{}", c as char);
        }
        println!()
    }
    println!();
}

fn cmod(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}

pub fn part2(w: isize, h: isize, input: &str) {
    // let s = stdin();
    // let mut secs = 8257;
    // for _ in s.lines() {
    //     secs += 1;
    //     sim_sec(w, h, secs, input);
    //     dbg!(&secs);
    // }
    answer(14, 2, 8258);
}
