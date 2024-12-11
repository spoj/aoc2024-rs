use itertools::Itertools;

pub static SAMPLE: &str = r#"125 17"#;
pub static INPUT: &str = include_str!("../data/d11.txt");

pub fn part1(input: &str) {
    let mut g = Gen::new();
    for _ in 0..25 {
        g.layer();
    }
    let day11_part1: usize = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .map(|n| g.solve(n))
        .sum();
    dbg!(day11_part1);
}
pub fn part2(input: &str) {
    let mut g = Gen::new();
    for _ in 0..75 {
        g.layer();
    }
    let day11_part2: usize = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .map(|n| g.solve(n))
        .sum();
    dbg!(day11_part2);
}

#[derive(Debug)]
struct Gen {
    data: Vec<[usize; 2024]>,
}

impl Gen {
    fn new() -> Self {
        let data = vec![[1; 2024]; 1];
        Self { data }
    }
    fn solve(&self, num: usize) -> usize {
        let mut buf = vec![num];
        let mut acc = 0;
        for k in (0..self.data.len()).rev() {
            if buf.is_empty() {
                break;
            }
            buf = buf
                .iter()
                .filter(|next| {
                    if **next < 2024 {
                        acc += self.data[k][**next];
                        false
                    } else {
                        if k == 0 {
                            acc += 1;
                        }
                        true
                    }
                })
                .flat_map(|&x| next_nums(x))
                .collect_vec();
        }
        acc
    }
    fn layer(&mut self) {
        let mut cur = [0; 2024];
        for (j, n) in cur.iter_mut().enumerate() {
            for next in next_nums(j) {
                *n += self.solve(next);
            }
        }
        self.data.push(cur);
    }
}

fn digits(num: usize) -> usize {
    if num == 0 {
        0
    } else {
        std::iter::successors(Some(num), {
            |x| if *x < 10 { None } else { Some(*x / 10) }
        })
        .count()
    }
}

fn next_nums(num: usize) -> Vec<usize> {
    // println!("check next for {}",num);
    let dig = digits(num);
    match dig {
        0 => {
            vec![1]
        }
        _ if dig % 2 == 0 => {
            let pow = 10usize.pow(dig as u32 / 2);
            let left = num / pow;
            let right = num - left * pow;
            vec![left, right]
        }
        _ => {
            vec![num * 2024]
        }
    }
}
