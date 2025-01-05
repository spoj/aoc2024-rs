use itertools::Itertools;

pub static SAMPLE: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;
pub static INPUT: &str = include_str!("../data/d25.txt");

pub fn part1(input: &str) -> usize {
    let mut keys: Vec<_> = vec![];
    let mut locks: Vec<_> = vec![];
    input.split("\n\n").for_each(|blk| {
        let rows = blk.lines().map(|l| l.as_bytes()).collect_vec();
        let mut nums = [0; 5];
        for row in rows.iter() {
            for (i, n) in nums.iter_mut().enumerate() {
                if row[i] == b'#' {
                    *n += 1;
                }
            }
        }
        for i in nums.iter_mut() {
            *i -= 1;
        }

        if rows[0].iter().all(|x| *x == b'#') {
            locks.push(nums);
        } else {
            keys.push(nums);
        }
    });
    let ans: usize = keys
        .iter()
        .map(|k| locks.iter().filter(|l| compat(*k, **l)).count())
        .sum();
    ans
}

fn compat(k: [i32; 5], l: [i32; 5]) -> bool {
    k.iter().zip(l).all(|(a, b)| a + b <= 5)
}
