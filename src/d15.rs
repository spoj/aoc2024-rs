use std::iter;

use itertools::Itertools;

pub static SAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;
pub static INPUT: &str = include_str!("../data/d15.txt");
static UP: u8 = b'^';
static DOWN: u8 = b'v';
static LEFT: u8 = b'<';
static RIGHT: u8 = b'>';

static BOT: u8 = b'@';
static WALL: u8 = b'#';
static FOOD: u8 = b'O';
static BOX1: u8 = b'[';
static BOX2: u8 = b']';
static EMPTY: u8 = b'.';

#[derive(Clone, Debug)]
struct Board {
    data: Vec<u8>,
    w: isize,
    _h: isize,
}

impl Board {
    fn from1(input: Board) -> Self {
        let data = input
            .data
            .into_iter()
            .flat_map(|c| match c {
                x if x == FOOD => b"[]",
                x if x == BOT => b"@.",
                x if x == EMPTY => b"..",
                x if x == WALL => b"##",
                _ => unreachable!(),
            })
            .copied()
            .collect_vec();
        let w = input.w * 2;
        let _h = input._h;
        Self { data, w, _h }
    }

    fn new(input: Vec<Vec<u8>>) -> Self {
        let data = input.concat();
        let data: Vec<u8> = data.into_iter().collect();
        let w = input[0].len() as isize;
        let h = input.len() as isize;
        Self { data, w, _h: h }
    }

    fn dir(&self, dir: u8) -> isize {
        match dir {
            d if d == UP => -self.w,
            d if d == DOWN => self.w,
            d if d == LEFT => -1,
            d if d == RIGHT => 1,
            _ => unreachable!(),
        }
    }

    fn parse(input: &str) -> Self {
        let input = input
            .lines()
            .map(|s| {
                iter::empty()
                    // .chain(vec![0; s.bytes().len()])
                    .chain(s.bytes())
                    // .chain(vec![0; s.bytes().len()])
                    .collect_vec()
            })
            .collect_vec();
        // input.insert(0, vec![0; input[0].len()]);
        // input.push(vec![0; input[0].len()]);
        Board::new(input)
    }
    fn try_move(&mut self, loc: isize, mv: u8) -> bool {
        let next = loc + self.dir(mv);
        match self.data[next as usize] {
            x if x == WALL => false,
            x if x == FOOD => {
                if self.try_move(next, mv) {
                    self.try_move(loc, mv)
                } else {
                    false
                }
            }
            x if x == EMPTY => {
                self.data[next as usize] = self.data[loc as usize];
                self.data[loc as usize] = EMPTY;
                true
            }
            x => {
                println!("got a {}", x as char);
                unreachable!()
            }
        }
    }
    fn front(&self, loc: isize, mv: u8) -> Vec<isize> {
        match self.data[loc as usize] {
            t if t == BOT => vec![loc + self.dir(mv)],
            t if t == WALL => vec![],
            t if t == EMPTY => vec![],
            t if t == BOX1 => match mv {
                m if m == LEFT => vec![loc + self.dir(mv)],
                m if m == RIGHT => vec![loc + self.dir(mv) + self.dir(mv)],
                m if m == UP => vec![loc + self.dir(mv), loc + self.dir(RIGHT) + self.dir(mv)],
                m if m == DOWN => vec![loc + self.dir(mv), loc + self.dir(RIGHT) + self.dir(mv)],
                _ => unreachable!(),
            },
            t if t == BOX2 => match mv {
                m if m == LEFT => vec![loc + self.dir(mv) + self.dir(mv)],
                m if m == RIGHT => vec![loc + self.dir(mv)],
                m if m == UP => vec![loc + self.dir(mv), loc + self.dir(LEFT) + self.dir(mv)],
                m if m == DOWN => vec![loc + self.dir(mv), loc + self.dir(LEFT) + self.dir(mv)],
                _ => unreachable!(),
            },
            t => vec![],
        }
    }

    fn body(&self, loc: isize) -> Vec<isize> {
        match self.data[loc as usize] {
            t if t == BOT => vec![loc],
            t if t == WALL => vec![loc],
            t if t == EMPTY => vec![loc],
            t if t == BOX1 => vec![loc, loc + self.dir(RIGHT)],
            t if t == BOX2 => vec![loc, loc + self.dir(LEFT)],
            _ => vec![],
        }
    }
    fn find_bot(&self) -> isize {
        self.data.iter().position(|x| *x == BOT).unwrap() as isize
    }

    fn pretty(&self) {
        for (i, v) in self.data.iter().enumerate() {
            if i % self.w as usize == 0 {
                println!();
            }
            print!("{}", *v as char);
        }
        println!();
    }
    fn gps_sum(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if *v == FOOD {
                    i / self.w as usize * 100 + i % self.w as usize
                } else {
                    0
                }
            })
            .sum()
    }
}

pub fn part1(input: &str) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut board = Board::parse(map);
    let mut cursor = board.find_bot();
    for mv in moves
        .bytes()
        .filter(|mv| [UP, DOWN, LEFT, RIGHT].contains(mv))
    {
        if board.try_move(cursor, mv) {
            cursor += board.dir(mv);
        }
    }
    board.pretty();
    dbg!(board.gps_sum());
}
pub fn part2(input: &str) {
    println!("{}", input);
}
