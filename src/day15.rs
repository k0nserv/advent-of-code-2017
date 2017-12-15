const DIVIDER: i64 = 2147483647;
const FACTOR_A: i64 = 16807;
const FACTOR_B: i64 = 48271;

#[derive(Debug)]
struct Generator {
    factor: i64,
    previous_value: i64,
    divisor: i64,
}

impl Generator {
    fn new(initial_value: i64, factor: i64, divisor: Option<i64>) -> Self {
        Generator {
            factor,
            previous_value: initial_value,
            divisor: divisor.unwrap_or(1),
        }
    }
}

impl Iterator for Generator {
    type Item = Option<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.previous_value * self.factor) % DIVIDER;
        self.previous_value = next;

        if next % self.divisor == 0 {
            Some(Some(next))
        } else {
            Some(None)
        }
    }
}

fn find_equal_pairs(v1: i64, v2: i64) -> Option<()> {
    let pair = ((v1 & 0xFFFF) as u16, (v2 & 0xFFFF) as u16);

    if pair.0 == pair.1 {
        return Some(());
    }

    None
}

pub fn solve(initial_value_a: i64, initial_value_b: i64) -> usize {
    let gen_a = Generator::new(initial_value_a, FACTOR_A, None);
    let gen_b = Generator::new(initial_value_b, FACTOR_B, None);

    gen_a
        .zip(gen_b)
        .take(40_000_000)
        .filter_map(|(v1, v2)| find_equal_pairs(v1.unwrap(), v2.unwrap()))
        .count()
}

pub fn solve2(initial_value_a: i64, initial_value_b: i64) -> usize {
    let gen_a = Generator::new(initial_value_a, FACTOR_A, Some(4));
    let gen_b = Generator::new(initial_value_b, FACTOR_B, Some(8));

    gen_a
        .filter(|v| v.is_some())
        .zip(gen_b.filter(|v| v.is_some()))
        .take(5_000_000)
        .filter_map(|(v1, v2)| find_equal_pairs(v1.unwrap(), v2.unwrap()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_five() {
        let gen_a = Generator::new(65, FACTOR_A, None);
        let gen_b = Generator::new(8921, FACTOR_B, None);

        assert_eq!(
            //https://open.spotify.com/track/5p6me2mwQrGfH30eExHn6v
            gen_a.take(5).collect::<Vec<_>>(),
            [
                Some(1092455),
                Some(1181022009),
                Some(245556042),
                Some(1744312007),
                Some(1352636452)
            ]
        );
    }

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve(65, 8921), 588);
    }

    #[test]
    fn test_cases_star_two() {
        assert_eq!(solve2(65, 8921), 309);
    }
}
