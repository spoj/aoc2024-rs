use itertools::Itertools;

pub static SAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
pub static INPUT: &str = include_str!("../data/d07.txt");


pub fn part1(input: &str) {
    let input = input
        .lines()
        .map(|st| {
            let (a, b) = st.split_once(": ").unwrap();
            let a: isize = a.parse().unwrap();
            let b: Vec<isize> = b
                .split_ascii_whitespace()
                .map(|num_as_str| num_as_str.parse().unwrap())
                .collect_vec();
            (a, b)
        })
        .collect_vec();
    let day7_part1: isize = input
        .iter()
        .filter(|(target, nums)| composable(*target, nums))
        .map(|(target, _)| target)
        .sum();
    dbg!(day7_part1);
}

fn composable(target: isize, nums: &[isize]) -> bool {
    if target == 0 && nums.is_empty() {
        true
    } else if target <= 0 || nums.is_empty() {
        false
    } else {
        let last = nums.last().unwrap();
        let nums_reduced = &nums[0..nums.len() - 1];
        composable(target - last, nums_reduced)
            || target % last == 0 && composable(target / last, nums_reduced)
    }
}
fn composable2(target: isize, nums: &[isize]) -> bool {
    if target == 0 && nums.is_empty() {
        true
    } else if target <= 0 || nums.is_empty() {
        false
    } else {
        let last = nums.last().unwrap();
        let nums_reduced = &nums[0..nums.len() - 1];
        composable2(target - last, nums_reduced)
            || target % last == 0 && composable2(target / last, nums_reduced)
            || {
                let st_last = format!("{}", last);
                format!("{}", target).ends_with(&st_last)
                    && composable2(target / 10_isize.pow(st_last.len() as u32), nums_reduced)
            }
    }
}

pub fn part2(input: &str) {
    let input = input
        .lines()
        .map(|st| {
            let (a, b) = st.split_once(": ").unwrap();
            let a: isize = a.parse().unwrap();
            let b: Vec<isize> = b
                .split_ascii_whitespace()
                .map(|num_as_str| num_as_str.parse().unwrap())
                .collect_vec();
            (a, b)
        })
        .collect_vec();
    let day7_part2: isize = input
        .iter()
        .filter(|(target, nums)| composable2(*target, nums))
        .map(|(target, _)| target)
        .sum();
    dbg!(day7_part2);
}
