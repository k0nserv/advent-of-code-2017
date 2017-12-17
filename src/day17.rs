use std::collections::hash_map::DefaultHasher;

pub fn solve(step_size: usize) -> usize {
    let mut buffer = Vec::with_capacity(2018);
    buffer.push(0);
    let mut next_value = 1;
    let mut current_position = 0;

    while next_value < 2018 {
        let next_position = (current_position + step_size + 1) % buffer.len();
        current_position = next_position;
        buffer.insert(next_position, next_value);
        next_value += 1;
    }

    buffer[current_position + 1]
}

pub fn solve2(step_size: usize) -> usize {
    let mut length = 1;
    let mut next_value = 1;
    let mut current_position: usize = 0;
    let mut index_one_value = 0;

    while next_value <= 50_000_000 {
        let next_position = (current_position + step_size + 1) % length;
        current_position = next_position;
        if current_position == 0 {
            index_one_value = next_value;
        }
        length += 1;
        next_value += 1;
    }

    index_one_value
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve(3), 638);
    }

    #[test]
    fn test_cases_star_two() {}
}
