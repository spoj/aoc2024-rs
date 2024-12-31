use std::collections::HashSet;

use itertools::Itertools;

use crate::answer;

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
    answer(5, 1, day5_part1);
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

fn topo_sort(orders: &[(isize, isize)], nodes: &[isize]) -> Option<Vec<isize>> {
    let mut nodes: HashSet<isize> = nodes.iter().copied().collect();
    let mut edges: Vec<(isize, isize)> = orders
        .iter()
        .copied()
        .filter(|(a, b)| nodes.contains(a) && nodes.contains(b))
        .collect();
    let mut ans: Vec<isize> = vec![];
    let mut s: Vec<isize> = nodes
        .iter()
        .copied()
        .filter(|x| edges.iter().all(|(_a, b)| x != b))
        .collect();

    while let Some(n) = s.pop() {
        edges.retain(|(a, _b)| n != *a);
        nodes.remove(&n);
        ans.push(n);
        s.extend(
            nodes
                .iter()
                .copied()
                .filter(|x| edges.iter().all(|(_a, b)| x != b)),
        );
    }

    if edges.is_empty() { Some(ans) } else { None }
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
        .flat_map(|report| topo_sort(&orders, report))
        .map(|report| midnum(&report))
        .sum();
    answer(5, 2, day5_part2);
}
