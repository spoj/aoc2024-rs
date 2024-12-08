use regex::Regex;

pub static SAMPLE: &str =
    r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

pub static SAMPLE2: &str =
    r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

pub static INPUT: &str = include_str!("../data/d03.txt");

pub fn part1(input: &str) {
    let re = Regex::new("mul\\(([0-9]+),([0-9]+)\\)").unwrap();
    let day3_part1: usize = re
        .captures_iter(input)
        .map(|cap| {
            let (_, [a, b]) = cap.extract();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();
            a * b
        })
        .sum();
    dbg!(day3_part1);
}

pub fn part2(input: &str) {
    let re = Regex::new("don't\\(\\)()()|do\\(\\)()()|mul\\(([0-9]+),([0-9]+)\\)").unwrap();
    let mut doing = true;
    let day3_part2: usize = re
        .captures_iter(input)
        .map(|cap| {
            let (ab, [a, b]) = cap.extract();
            match ab {
                "don't()" => {
                    doing = false;
                    0
                }
                "do()" => {
                    doing = true;
                    0
                }
                _ if doing => {
                    let a: usize = a.parse().unwrap();
                    let b: usize = b.parse().unwrap();
                    a * b
                }
                _ => 0,
            }
        })
        .sum();
    dbg!(day3_part2);
}
