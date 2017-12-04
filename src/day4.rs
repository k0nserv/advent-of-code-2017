use std::iter::FromIterator;
use std::collections::HashSet;

trait Validator {
    fn is_valid(&self, phrase: &Vec<&str>) -> bool;
}

struct UniquenessValidator {}

impl Validator for UniquenessValidator {
    fn is_valid(&self, phrase: &Vec<&str>) -> bool {
        let len = &phrase.len();
        let mut set: HashSet<&str> = HashSet::with_capacity(*len);
        set.extend(phrase);

        set.len() == *len
    }
}

struct AnagramValidator {}

impl Validator for AnagramValidator {
    fn is_valid(&self, phrase: &Vec<&str>) -> bool {
        let sorted = phrase.iter().map(|&word| {
                                           let mut chars = word.chars().collect::<Vec<char>>();
                                           chars.sort_by(|a, b| b.cmp(a));

                                           String::from_iter(chars)
                                       });

        let len = sorted.len();

        let mut set: HashSet<String> = HashSet::with_capacity(len);
        set.extend(sorted);

        set.len() == len
    }
}

pub fn parse(input: &str) -> Vec<Vec<&str>> {
    input.trim()
        .split("\n")
        .map(|phrase| phrase.split_whitespace())
        .map(|words| words.collect::<Vec<&str>>())
        .collect()
}

pub fn solve_star_one(input: &str) -> u32 {
    let phrases = parse(input);
    let validator = UniquenessValidator {};

    phrases.iter()
        .map(|phrase| validator.is_valid(phrase))
        .filter(|&valid| valid)
        .collect::<Vec<bool>>()
        .len() as u32
}

pub fn solve_star_two(input: &str) -> u32 {
    let phrases = parse(input);
    let validator = AnagramValidator {};

    phrases.iter()
        .map(|phrase| validator.is_valid(phrase))
        .filter(|&valid| valid)
        .collect::<Vec<bool>>()
        .len() as u32
}

#[cfg(test)]
mod tests {
    use super::{solve_star_one, solve_star_two};

    #[test]
    fn test_cases_star_one() {
        let input = "
            aa bb cc dd ee
            aa bb cc dd aa
            aa bb cc dd aaa
            ";
        assert_eq!(solve_star_one(input), 2);
    }

    #[test]
    fn test_cases_star_two() {
        let input = "
            abcde fghij
            abcde xyz ecdab
            a ab abc abd abf abj
            iiii oiii ooii oooi oooo
            oiii ioii iioi iiio
            ";
        assert_eq!(solve_star_two(input), 3);
    }
}
