use std::collections::HashSet;

pub fn solve(input: &str) -> u32 {
    input.trim().split("\n").map(|phrase| phrase.split_whitespace()).map(|words| {
        let vec = words.collect::<Vec<&str>>();
        let len = &vec.len();
        let mut set: HashSet<&str> = HashSet::with_capacity(*len);
        set.extend(vec);

        set.len() == *len
    }).filter(|&valid| valid).collect::<Vec<bool>>().len() as u32
}

#[cfg(test)]
mod tests {
    use super::{solve};

    #[test]
    fn test_cases_star_one() {
        let input = "
            aa bb cc dd ee
            aa bb cc dd aa
            aa bb cc dd aaa
            ";
        assert_eq!(solve(input), 2);
    }
}
