use itertools::Itertools;

pub static SAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

pub static INPUT: &str = include_str!("../data/d02.txt");

pub fn part1(input: &str) {
    let day2_part1 = input
        .lines()
        .filter(|rpt_as_str| {
            let rpt: Vec<isize> = rpt_as_str
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec();
            strict_safe(&rpt)
        })
        .count();
    dbg!(day2_part1);
}

pub fn part2(input: &str) {
    let day2_part2 = input
        .lines()
        .filter(|rpt_as_str| {
            let rpt: Vec<isize> = rpt_as_str
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec();
            relax_safe(&rpt)
        })
        .count();
    dbg!(day2_part2);
}

fn strict_safe(report: &[isize]) -> bool {
    // count_fast!('.');

    let all_dec = report.windows(2).all(|ab| {
        let c = ab[0] - ab[1];
        c == 1 || c == 2 || c == 3
    });
    let all_inc = report.windows(2).all(|ab| {
        let c = ab[0] - ab[1];
        c == -1 || c == -2 || c == -3
    });
    all_inc || all_dec
}

fn relax_safe(report: &[isize]) -> bool {
    // count_slow!('#');

    let mut copy = report[1..].to_owned();
    if strict_safe(&copy) {
        return true;
    }
    for i in 0..copy.len() {
        copy[i] = report[i];
        if strict_safe(&copy) {
            return true;
        }
    }
    false
}
