
use itertools::Itertools;

pub static SAMPLE: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;
pub static INPUT: &str = include_str!("../data/d17.txt");

#[derive(Debug)]
struct Machine {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
    cursor: usize,
}

impl Machine {
    fn new(a: usize, b: usize, c: usize, program: Vec<usize>) -> Self {
        Self {
            a,
            b,
            c,
            program,
            cursor: 0,
        }
    }
    fn parse(input: &str) -> Self {
        let (reg, prog) = input.split_once("\n\n").unwrap();
        let reg = reg
            .lines()
            .map(|line| {
                let (_, right) = line.split_once(": ").unwrap();
                right.parse().unwrap()
            })
            .collect_vec();
        let prog = prog
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(parse_3bit)
            .collect_vec();
        Self::new(reg[0], reg[1], reg[2], prog)
    }
    fn combo(&self, op: usize) -> usize {
        match op {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
    fn run_once(&mut self) -> Option<usize> {
        match self.program[self.cursor] {
            0 => {
                let num = self.a;
                let den = 2usize.pow(self.combo(self.program[self.cursor + 1]) as u32);
                let res = num / den;
                self.a = res;
                self.cursor += 2;
            }
            1 => {
                let left = self.b;
                let right = self.program[self.cursor + 1];
                let res = left ^ right;
                self.b = res;
                self.cursor += 2;
            }
            2 => {
                let op = self.combo(self.program[self.cursor + 1]);
                let res = op % 8;
                self.b = res;
                self.cursor += 2;
            }
            3 => {
                if self.a != 0 {
                    self.cursor = self.program[self.cursor + 1];
                } else {
                    self.cursor += 2;
                }
            }
            4 => {
                let left = self.b;
                let right = self.c;
                let res = left ^ right;
                self.b = res;
                self.cursor += 2;
            }
            5 => {
                let out = self.combo(self.program[self.cursor + 1]);
                let out = out % 8;
                self.cursor += 2;
                return Some(out);
            }
            6 => {
                let num = self.a;
                let den = 2usize.pow(self.combo(self.program[self.cursor + 1]) as u32);
                let res = num / den;
                self.b = res;
                self.cursor += 2;
            }
            7 => {
                let num = self.a;
                let den = 2usize.pow(self.combo(self.program[self.cursor + 1]) as u32);
                let res = num / den;
                self.c = res;
                self.cursor += 2;
            }
            _ => {}
        }
        None
    }
    fn complete(&mut self) -> Vec<usize> {
        let mut out = vec![];
        while self.cursor < self.program.len() {
            if let Some(x) = self.run_once() {
                out.push(x)
            }
        }
        out
    }
}

fn parse_3bit(input: &str) -> usize {
    usize::from_str_radix(input.trim(), 8).unwrap()
}
fn print_3bit(input: usize) -> String {
    format!("{:o}", input)
}

pub fn part1(input: &str) {
    let mut x = Machine::parse(input);
    let list = x.complete();
    let day17_part1 = list.into_iter().map(print_3bit).join(",");
    dbg!(day17_part1);
}
pub fn part2(input: &str) {
    println!("{}", input);
}
