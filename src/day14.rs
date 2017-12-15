use std::collections::{HashMap, VecDeque};

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
        .flat_map(move |(x, &v)| {
            (0..128).rev().map(move |y| {
                let is_empty = ((v >> y) & (0b1 as u128)) as u32 == 0;
                ((x, 127 - y), is_empty)
            })
        })
        .collect::<HashMap<_, _>>();
    let mut num_regions = 0;
    let neighbours: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

    while accounted_for.values().any(|&v| v == false) {
        let clone = accounted_for.clone();
        let next = clone.iter().find(|&(_, &v)| v == false);

        let mut to_visit = VecDeque::<(usize, usize)>::new();
        to_visit.push_back(*next.unwrap().0);

        while !to_visit.is_empty() {
            let current = to_visit.pop_front().unwrap();
            accounted_for.insert(current, true);

            neighbours.iter().for_each(|&(x, y)| {
                let neighbour_loc: (i32, i32) = (current.0 as i32 + x, current.1 as i32 + y);
                if neighbour_loc.0 < 0 || neighbour_loc.1 < 0 {
                    return;
                }

                if let Some(neighbour) =
                    accounted_for.get(&(neighbour_loc.0 as usize, neighbour_loc.1 as usize))
                {
                    if *neighbour == false {
                        to_visit.push_back((neighbour_loc.0 as usize, neighbour_loc.1 as usize));
                    }
                }
            });
        }
        num_regions += 1;
    }

    num_regions
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
