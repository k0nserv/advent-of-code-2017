use std::collections::HashMap;

fn parse(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|v| v.parse::<u32>().expect("Expected only numbers"))
        .collect()
}

fn identify_next(values: &Vec<u32>) -> (usize, u32) {
    let mut max = 0;
    let mut index = 0;

    for (i, &v) in values.iter().enumerate() {
        if v > max {
            max = v;
            index = i;
        }
    }

    (index, max)
}

pub fn solve(input: &str) -> (u32, u32) {
    let parsed = parse(input);

    let input_len = parsed.len();
    let mut current_memory = parsed;
    let mut seen: HashMap<Vec<u32>, u32> = HashMap::new();
    seen.insert(current_memory.clone(), 0);
    let mut cycles = 0;
    let mut loop_size = 0;

    loop {
        let next = identify_next(&current_memory);

        current_memory[next.0] = 0;
        let mut i = next.0 + 1;
        let mut left = next.1;

        while left > 0 {
            current_memory[i % input_len] += 1;
            left -= 1;
            i += 1;
        }
        cycles += 1;

        if let Some(v) = seen.get(&current_memory) {
            loop_size = cycles - v;
            break;
        }
        seen.insert(current_memory.clone(), cycles);
    }

    (cycles, loop_size)
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve("0 2 7 0"), (5, 4));
    }

    #[test]
    fn test_cases_star_two() {
        assert_eq!(solve("0 2 7 0"), (5, 4));
    }
}
