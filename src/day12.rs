use std::collections::{HashMap, HashSet, VecDeque};

fn groups(connections: &HashMap<u32, Vec<u32>>) -> HashSet<Vec<u32>> {
    let mut result = HashSet::<Vec<u32>>::new();
    for (key, _) in connections {
        let mut programs = HashSet::<u32>::new();
        let mut queue = VecDeque::<u32>::new();
        queue.push_back(*key);

        while !queue.is_empty() {
            let id = queue.pop_front().unwrap();
            programs.insert(id);
            connections
                .get(&id)
                .unwrap_or(&Vec::<u32>::new())
                .iter()
                .for_each(|v| {
                    if !queue.contains(v) && !programs.contains(v) {
                        queue.push_back(*v)
                    }
                });
        }
        let mut vec = programs.into_iter().collect::<Vec<_>>();
        vec.sort();
        result.insert(vec);
    }

    result
}

pub fn solve(input: &str) -> u32 {
    let connections: HashMap<u32, Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let id = words[0].parse::<u32>().expect("Expected number");
            let connections = words[2..]
                .iter()
                .map(|v| {
                    v.trim_matches(|c: char| !c.is_numeric())
                        .parse::<u32>()
                        .expect("Expected number")
                })
                .collect::<Vec<_>>();

            (id, connections)
        })
        .collect();

    let mut programs = HashSet::<u32>::new();
    let mut queue = VecDeque::<u32>::new();
    queue.push_back(0);

    while !queue.is_empty() {
        let id = queue.pop_front().unwrap();
        programs.insert(id);
        connections
            .get(&id)
            .unwrap_or(&Vec::<u32>::new())
            .iter()
            .for_each(|v| {
                if !queue.contains(v) && !programs.contains(v) {
                    queue.push_back(*v)
                }
            });
    }

    programs.len() as u32
}

pub fn solve2(input: &str) -> u32 {
    let connections: HashMap<u32, Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let id = words[0].parse::<u32>().expect("Expected number");
            let connections = words[2..]
                .iter()
                .map(|v| {
                    v.trim_matches(|c: char| !c.is_numeric())
                        .parse::<u32>()
                        .expect("Expected number")
                })
                .collect::<Vec<_>>();

            (id, connections)
        })
        .collect();

    groups(&connections).len() as u32
}

#[cfg(test)]
mod tests {
    use super::{solve, solve2};

    #[test]
    fn test_cases_star_one() {
        let input = "
                0 <-> 2
                1 <-> 1
                2 <-> 0, 3, 4
                3 <-> 2, 4
                4 <-> 2, 3, 6
                5 <-> 6
                6 <-> 4, 5
                ";
        assert_eq!(solve(&input), 6);
    }

    #[test]
    fn test_cases_star_two() {
        let input = "
                0 <-> 2
                1 <-> 1
                2 <-> 0, 3, 4
                3 <-> 2, 4
                4 <-> 2, 3, 6
                5 <-> 6
                6 <-> 4, 5
                ";
        assert_eq!(solve2(&input), 2);
    }
}
