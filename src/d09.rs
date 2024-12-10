use std::iter::repeat_n;

use itertools::Itertools;

pub static SAMPLE: &str = r#"2333133121414131402"#;
pub static INPUT: &str = include_str!("../data/d09.txt");

pub fn part1(input: &str) {
    let input = input.bytes().map(|x| x - b'0').collect_vec();
    let forward = input
        .iter()
        .enumerate()
        .filter(|(a, _b)| a % 2 == 0)
        .map(|(_a, b)| b)
        .enumerate()
        .flat_map(|(a, b)| repeat_n(a, *b as usize))
        .collect_vec();
    let mut disk: Vec<usize> = vec![];
    let mut left = 0;
    let mut right = forward.len() - 1;
    'a: for (i, n) in input.iter().enumerate() {
        let is_fwd = i % 2 == 0;
        for _ in 0..*n {
            if is_fwd {
                disk.push(forward[left]);
                left += 1;
            } else {
                disk.push(forward[right]);
                right -= 1;
            }
            if left > right {
                break 'a;
            }
        }
    }
    let day9_part1: usize = disk.iter().enumerate().map(|(a, b)| a * b).sum();
    dbg!(day9_part1);
}

pub fn part2(input: &str) {
    let input = input.bytes().map(|x| x - b'0').collect_vec();
    let mut data: Vec<(usize, usize, usize)> = vec![]; // (start, length, data)
    let mut free: Vec<(usize, usize)> = vec![]; // (start, length)
    let mut pos = 0;
    input.iter().enumerate().for_each(|(index, len)| {
        if index % 2 == 0 {
            //  this is a data extent
            data.push((pos, *len as usize, index / 2));
        } else {
            // free extend
            free.push((pos, *len as usize));
        }
        pos += *len as usize;
    });
    for (data_start, data_len, _data_id) in data.iter_mut().rev() {
        if let Some((free_start, free_len)) = free
            .iter_mut()
            .find(|(fs, fl)| fl >= data_len && fs < data_start)
        {
            *data_start = *free_start;
            *free_start += *data_len;
            *free_len -= *data_len;
        }
    }
    let day9_part2: usize = data
        .into_iter()
        .map(|(start, len, data)| (start + start + len - 1) * len / 2 * data)
        .sum();
    dbg!(day9_part2);
}
