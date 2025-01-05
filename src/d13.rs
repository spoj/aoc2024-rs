use itertools::Itertools;
use regex::Regex;

pub static SAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;
pub static INPUT: &str = include_str!("../data/d13.txt");

pub fn part1(input: &str) -> isize {
    let re = Regex::new(
        r"(?s)X\+([0-9]+).*Y\+([0-9]+).*X\+([0-9]+).*Y\+([0-9]+).*X\=([0-9]+).*Y\=([0-9]+)",
    )
    .unwrap();
    let machines = input
        .split("\n\n")
        .map(|machine_str| {
            let (_, nums): (_, [&str; 6]) = re.captures(machine_str).unwrap().extract();
            nums.map(|x| x.parse::<isize>().unwrap())
        })
        .collect_vec();

    machines
        .into_iter()
        .filter_map(|[a, b, c, d, e, f]| {
            // println!("det is {}",a*d-b*c);

            (0..e / a)
                .flat_map(|i| {
                    let e_rem = e - a * i;
                    let j = e_rem / c;

                    if i * a + j * c == e && i * b + j * d == f {
                        // println!("sol found at {} {}",i,j);
                        Some(i * 3 + j)
                    } else {
                        None
                    }
                })
                .min()
        })
        .sum::<isize>()
}

pub fn part2(input: &str) -> isize {
    let re = Regex::new(
        r"(?s)X\+([0-9]+).*Y\+([0-9]+).*X\+([0-9]+).*Y\+([0-9]+).*X\=([0-9]+).*Y\=([0-9]+)",
    )
    .unwrap();
    let machines = input
        .split("\n\n")
        .map(|machine_str| {
            let (_, nums): (_, [&str; 6]) = re.captures(machine_str).unwrap().extract();
            nums.map(|x| x.parse::<isize>().unwrap())
        })
        .collect_vec();
    let day13_part2: isize = machines
        .into_iter()
        .flat_map(|[a, b, c, d, e, f]| {
            let f = f + 10000000000000;
            let e = e + 10000000000000;
            // println!("det is {}",a*d-b*c);
            let i = (e * d - f * c) / (a * d - b * c);
            let j = (a * f - e * b) / (a * d - b * c);
            if i * a + j * c == e && i * b + j * d == f {
                Some(i * 3 + j)
            } else {
                None
            }
        })
        .sum();
    day13_part2
}
