use std::collections::{HashMap, VecDeque, hash_map::Entry};

pub static SAMPLE: &str = r#"029A"#;
pub static INPUT: &str = include_str!("../data/d00.txt");

static BOARD_STR: &str = r"
789
456
123
 0A";

static UP: u8 = b'^';
static DOWN: u8 = b'v';
static LEFT: u8 = b'<';
static RIGHT: u8 = b'>';
static A: u8 = b'A';
static SPACE: u8 = b' ';

#[derive(Clone, Debug)]
struct Dpad {}

impl Dpad {
    fn new() -> Self {
        Self {}
    }

    fn nexts(&self, from: u8) -> Vec<(u8, u8)> {
        // output: (direction, next value)
        match from {
            b'A' => vec![(UP, b'3'), (LEFT, b'0')],
            b'0' => vec![(UP, b'2'), (RIGHT, b'A')],
            b'1' => vec![(UP, b'4'), (RIGHT, b'2')],
            b'2' => vec![(UP, b'5'), (LEFT, b'1'), (DOWN, b'0'), (RIGHT, b'3')],
            b'3' => vec![(UP, b'6'), (LEFT, b'2'), (DOWN, b'A')],
            b'4' => vec![(UP, b'7'), (RIGHT, b'5'), (DOWN, b'1')],
            b'5' => vec![(UP, b'8'), (LEFT, b'4'), (DOWN, b'2'), (RIGHT, b'6')],
            b'6' => vec![(UP, b'9'), (LEFT, b'5'), (DOWN, b'3')],
            b'7' => vec![(DOWN, b'4'), (RIGHT, b'8')],
            b'8' => vec![(LEFT, b'7'), (DOWN, b'5'), (RIGHT, b'9')],
            b'9' => vec![(LEFT, b'8'), (DOWN, b'6')],
            _ => unreachable!(),
        }
    }

    fn short(&self, from: u8, to: u8) -> Vec<u8> {
        let mut done: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((vec![], from));
        while let Some((path, node)) = q.pop_front() {
            if let Entry::Vacant(e) = done.entry(node) {
                e.insert(path.clone());
                q.extend(self.nexts(node).iter().map(|i| {
                    let mut newpath = path.clone();
                    newpath.push(i.0);
                    (newpath, i.1)
                }));
            }
        }
        done[&to].clone()
    }
}

pub fn part1(input: &str) {
    let d = Dpad::new();
    d.short(b'A', b'5')
        .iter()
        .for_each(|b| println!("{}", *b as char));
    d.short(b'0', b'9')
        .iter()
        .for_each(|b| println!("{}", *b as char));

    dbg!(d.short(b'0', b'9'));
}

pub fn part2(input: &str) {
    dbg!(input);
}
