use std::ops::Range;
use std::collections::HashMap;

#[derive(Debug)]
struct State {
    levels: HashMap<u32, u32>,
    scanner_locations: HashMap<u32, (u32, i32)>,
    packect_location: i32,
    severity: u32,
    final_location: u32,
}

impl State {
    fn parse(input: &str) -> Self {
        let levels = input
            .trim()
            .lines()
            .map(|line| {
                let parts = line.trim().split(':').collect::<Vec<_>>();
                (
                    parts[0].parse::<u32>().unwrap(),
                    parts[1].trim().parse::<u32>().unwrap(),
                )
            })
            .collect::<HashMap<_, _>>();

        let scanner_locations = levels
            .keys()
            .map(|level| (*level, (0, 1)))
            .collect::<HashMap<_, _>>();

        let final_location: u32 = *levels.keys().max().unwrap_or(&0);

        State {
            levels,
            scanner_locations,
            packect_location: -1,
            severity: 0,
            final_location,
        }
    }

    fn advance(&mut self) {
        self.packect_location += 1;
        let packect_location = self.packect_location as u32;
        if let Some(location) = self.scanner_locations.get(&packect_location) {
            if location.0 == 0 {
                self.severity +=
                    packect_location * *self.levels.get(&packect_location).unwrap_or(&0);
            }
        }

        for (level, range) in &self.levels {
            let &(current_location, next_step) = self.scanner_locations.get(level).unwrap();
            let new_location = ((current_location as i32) + next_step) as u32;

            let new_next_step = if new_location == range - 1 || new_location == 0 {
                -next_step
            } else {
                next_step
            };

            self.scanner_locations
                .insert(*level, (new_location, new_next_step));
        }
    }

    fn is_clear(delay: u32, range: u32) -> bool {
        delay % ((range - 1) * 2) != 0
    }

    fn clear_with_delay(&self, delay: u32) -> bool {
        self.levels
            .iter()
            .all(|(&level, &range)| Self::is_clear(delay + level, range))
    }

    fn at_end(&self) -> bool {
        (self.packect_location as u32) == self.final_location
    }
}

struct Counter {
    value: u32,
}

impl Counter {
    fn new(start_value: u32) -> Self {
        Counter { value: start_value }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.value;
        self.value += 1;
        Some(value)
    }
}

pub fn solve(input: &str) -> u32 {
    let mut state = State::parse(input);

    while !state.at_end() {
        state.advance();
    }

    state.severity
}

pub fn solve2(input: &str) -> u32 {
    let mut state = State::parse(input);
    let mut severity = 1;
    let mut delay = 10;

    Counter::new(10)
        .find(|&value| state.clear_with_delay(value))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{solve, solve2};

    #[test]
    fn test_cases_star_one() {
        let input = "
                0: 3
                1: 2
                4: 4
                6: 4
                ";
        assert_eq!(solve(&input), 24);
    }

    #[test]
    fn test_cases_star_two() {
        let input = "
                0: 3
                1: 2
                4: 4
                6: 4
                ";
        assert_eq!(solve2(&input), 10);
    }
}
