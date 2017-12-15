use std::collections::VecDeque;

use day10::knot_hash;

fn str_as_ascii_values(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|c| c as u8)
        .map(|c| c as usize)
        .collect::<Vec<usize>>()
}

fn build(input: &str) -> Vec<u128> {
    (0..128)
        .map(|i| {
            knot_hash(
                str_as_ascii_values(format!("{}-{}", input, i).as_ref()),
                256,
                64,
            )
        })
        .map(|hash| {
            hash.chars().enumerate().fold(0, |acc: u128, (i, c)| {
                let v = c.to_digit(16).unwrap();

                acc | ((v & 0xf) as u128) << (124 - i * 4)
            })
        })
        .collect::<Vec<u128>>()
}

pub fn solve(input: &str) -> u32 {
    let rows = build(input);

    rows.iter().fold(0, |acc, v| {
        acc + (0..128).fold(0, |inner_acc, i| {
            inner_acc + ((v >> i & (0b0001 as u128))) as u32
        })
    })
}

pub fn solve2(input: &str) -> u32 {
    let rows = build(input);
    let mut accounted_for = rows.iter()
        .enumerate()
        .map(|(x, v)| {
            (0..128)
                .rev()
                .map(|y| {
                    let is_empty = ((v >> y) & (0b1 as u128)) as u32 == 0;
                    (x, 127 - y, is_empty)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut num_regions = 0;
    let neighbours = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while accounted_for
        .iter()
        .any(|data| data.iter().any(|&(_, _, v)| v == false))
    {
        let mut to_visit = VecDeque::<(usize, usize)>::new();
        let next = accounted_for.iter().enumerate().map(|x, row| row.iter().enumerate().filter_map(|y, v|
        to_visit.push_back((0, 0));

        while !to_visit.is_empty() {
            let next = to_visit.pop_front().unwrap();
            accounted_for[next.0][next.1] = true;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::{solve, solve2};

    #[test]
    fn test_cases_star_one() {
        let input = "flqrgnkx";
        assert_eq!(solve(&input), 8108);
        assert_eq!(solve2(&input), 1242);
    }
}
