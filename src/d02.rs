use itertools::Itertools;

pub static SAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

pub static INPUT: &str = include_str!("../data/d02.txt");

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|rpt_as_str| {
            let rpt: Vec<isize> = rpt_as_str
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec();
            strict_safe(&rpt)
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|rpt_as_str| {
            let rpt: Vec<isize> = rpt_as_str
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec();
            relax_safe(&rpt)
        })
        .count()
}

fn strict_safe(report: &[isize]) -> bool {
    report.windows(3).all(|ab| {
        let d1 = ab[0] - ab[1];
        let d2 = ab[1] - ab[2];
        d1.abs() >= 1
            && d1.abs() <= 3
            && d2.abs() >= 1
            && d2.abs() <= 3
            && d1.signum() == d2.signum()
    })
}

fn relax_safe(report: &[isize]) -> bool {
    let mut copy = report[1..].to_owned();
    let mut i = 0;
    loop {
        if strict_safe(&copy) {
            return true;
        } else if i == copy.len() {
            break;
        }
        copy[i] = report[i];
        i += 1;
    }
    false
}
