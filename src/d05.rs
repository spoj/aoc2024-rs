use itertools::Itertools;

pub static SAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
pub static INPUT: &str = include_str!("../data/d05.txt");

pub fn part1(input: &str) {
    let (a, b) = input.split_once("\n\n").unwrap();
    let orders: Vec<(isize, isize)> = a
        .lines()
        .map(|line| {
            let (fst, snd) = line.split_once('|').unwrap();
            (fst.parse().unwrap(), snd.parse().unwrap())
        })
        .collect_vec();
    let reports: Vec<Vec<isize>> = b
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_as_str| num_as_str.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let day5_part1: isize = reports
        .into_iter()
        .filter(|report| sat_all(report, &orders))
        .map(|report| midnum(&report))
        .sum();
    dbg!(day5_part1);
}

fn sat_one(report: &[isize], order: &(isize, isize)) -> bool {
    let pos1 = report.iter().position(|n| *n == order.0);
    let pos2 = report.iter().position(|n| *n == order.1);
    match (pos1, pos2) {
        (Some(pos1), Some(pos2)) => pos1 < pos2,
        _ => true,
    }
}

fn sat_all(report: &[isize], orders: &[(isize, isize)]) -> bool {
    orders.iter().all(|order| sat_one(report, order))
}

fn top_sort_2(report: &[isize], orders: &[(isize, isize)]) -> Option<Vec<isize>> {
    fn top_sort2_int(report: &[isize], acc: &mut Vec<isize>, orders: &[(isize, isize)]) -> bool {
        if !sat_all(acc, orders) {
            return false;
        }

        if report.is_empty() {
            return true;
        }

        let acclen = acc.len();
        let next = report[0];
        for i in 0..acclen + 1 {
            acc.insert(i, next);
            let nextresult = top_sort2_int(&report[1..], acc, orders);
            if nextresult {
                return true;
            } else {
                acc.remove(i);
            }
        }

        false
    }
    let mut acc = vec![];
    let res = top_sort2_int(report, &mut acc, orders);
    if res { Some(acc) } else { None }
}

fn midnum(report: &[isize]) -> isize {
    report[report.len() / 2]
}

pub fn part2(input: &str) {
    let (a, b) = input.split_once("\n\n").unwrap();
    let orders: Vec<(isize, isize)> = a
        .lines()
        .map(|line| {
            let (fst, snd) = line.split_once('|').unwrap();
            (fst.parse().unwrap(), snd.parse().unwrap())
        })
        .collect_vec();
    let reports: Vec<Vec<isize>> = b
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_as_str| num_as_str.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let day5_part2: isize = reports
        .iter()
        .filter(|report| !sat_all(report, &orders))
        .flat_map(|report| top_sort_2(report, &orders))
        .map(|report| midnum(&report))
        .sum();
    dbg!(day5_part2);
}
