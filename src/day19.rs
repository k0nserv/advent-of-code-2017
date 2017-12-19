use std::collections::HashSet;

type Location = (usize, usize);

fn is_valid_location(location: &Location, grid: &Vec<Vec<char>>) -> bool {
    location.1 >= 0 && location.1 < grid.len() && location.0 >= 0
        && location.0 < grid[location.1].len()
}

pub fn solve(input: &str) -> (String, u64) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| line.len() != 0)
        .map(|line| line.chars().collect())
        .collect();

    let mut current_position = (
        grid[0]
            .iter()
            .position(|&c| c == '|')
            .expect("Expected `|` on the first line"),
        0,
    );
    let mut previous_positions = HashSet::<(usize, usize)>::new();
    let mut direction: (i64, i64) = (0, 1);
    let mut cont = true;
    let mut seen_letters = Vec::new();
    let mut count = 0;

    while cont {
        previous_positions.insert(current_position);
        let potential_next = (
            (current_position.0 as i64 + direction.0) as usize,
            (current_position.1 as i64 + direction.1) as usize,
        );
        let value = grid[current_position.1][current_position.0];
        match value {
            '|' | '-' => {
                current_position = potential_next;
            }
            '+' => {
                let left = (current_position.0 - 1, current_position.1);
                let right = (current_position.0 + 1, current_position.1);
                let top = (current_position.0, current_position.1 - 1);
                let bottom = (current_position.0, current_position.1 + 1);

                if is_valid_location(&potential_next, &grid)
                    && !grid[potential_next.1][potential_next.0].is_whitespace()
                {
                    current_position = potential_next;
                } else if is_valid_location(&left, &grid)
                    && !(grid[left.1][left.0].is_whitespace() || previous_positions.contains(&left))
                {
                    current_position = left;
                    direction = (-1, 0);
                } else if is_valid_location(&right, &grid)
                    && !(grid[right.1][right.0].is_whitespace()
                        || previous_positions.contains(&right))
                {
                    current_position = right;
                    direction = (1, 0);
                } else if is_valid_location(&top, &grid)
                    && !(grid[top.1][top.0].is_whitespace() || previous_positions.contains(&top))
                {
                    current_position = top;
                    direction = (0, -1);
                } else if is_valid_location(&bottom, &grid)
                    && !(grid[bottom.1][bottom.0].is_whitespace()
                        || previous_positions.contains(&bottom))
                {
                    current_position = bottom;
                    direction = (0, 1);
                } else {
                    assert!(false, "This is bad");
                }
            }
            _ => {
                if value.is_alphabetic() {
                    seen_letters.push(value);
                    current_position = potential_next;

                    if grid[current_position.1][current_position.0].is_whitespace() {
                        cont = false;
                    }
                } else {
                    assert!(false, "Shouldn't be here");
                }
            }
        }
        count += 1;
    }

    (seen_letters.iter().collect::<String>(), count)
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_cases_star_one() {
        let input = "
     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
     ";
        assert_eq!(solve(input), (String::from("ABCDEF"), 38));
    }
}
