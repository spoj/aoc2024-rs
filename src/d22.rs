pub static SAMPLE: &str = r#"1
10
100
2024
"#;
pub static INPUT: &str = include_str!("../data/d22.txt");

fn nextsec(from: usize) -> usize {
    let mix = |old, i| old ^ i;
    let prune = |i| i % 16777216;

    let a = prune(mix(from, from * 64));
    let b = prune(mix(a, a / 32));
    prune(mix(b, b * 2048))
}

fn gens(from: usize, steps: usize) -> usize {
    std::iter::successors(Some(from), |&x| Some(nextsec(x)))
        .nth(steps)
        .unwrap()
}

pub fn part1(input: &str) {
    println!("{}", input);
    let ans: usize = input.lines().map(|l| gens(l.parse().unwrap(), 2000)).sum();
    dbg!(ans);
}
pub fn part2(input: &str) {
    println!("{}", input);
}
