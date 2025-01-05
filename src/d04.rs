use itertools::Itertools;


pub static SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

pub static INPUT: &str = include_str!("../data/d04.txt");

pub fn part1(input: &str) -> usize {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|s| s.bytes().chain(*b".").collect_vec())
        .collect_vec();
    let w = input[0].len();
    let h = input.len();
    let input_one = input.concat();
    let ww = w as isize;
    let hh = h as isize;
    let dirs = [-ww - 1, -ww, -ww + 1, -1, 1, ww - 1, ww, ww + 1];
    (0..w * h)
        .flat_map(|at| {
            dirs.iter().map(move |dir| {
                let at = at as isize;
                [at, at + dir, at + 2 * dir, at + 3 * dir]
            })
        })
        .filter(|seq| seq[3] >= 0 && seq[3] < ww * hh)
        .filter(|seq| &seq.map(|loc| input_one[loc as usize]) == b"XMAS")
        .count()
}

pub fn part2(input: &str) -> usize {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|s| s.bytes().chain(*b".").collect_vec())
        .collect_vec();
    let w = input[0].len();
    let h = input.len();
    let input_one = input.concat();
    let ww = w as isize;
    let hh = h as isize;
    (0..w * h)
        .flat_map(|at| {
            let at = at as isize;
            [
                [at, at - ww - 1, at - ww + 1, at + ww - 1, at + ww + 1],
                [at, at - ww - 1, at + ww - 1, at - ww + 1, at + ww + 1],
                [at, at + ww + 1, at - ww + 1, at + ww - 1, at - ww - 1],
                [at, at + ww + 1, at + ww - 1, at - ww + 1, at - ww - 1],
            ]
        })
        .filter(|seq| seq.iter().all(|&loc| loc >= 0 && loc < ww * hh))
        .filter(|seq| &seq.map(|loc| input_one[loc as usize]) == b"AMMSS")
        .count()
}
