use std::iter::FromIterator;
use std::collections::HashSet;

pub trait Validator {
    fn new() -> Self;
    fn is_valid(&self, phrase: &Vec<&str>) -> bool;
}

pub struct UniquenessValidator {}

impl Validator for UniquenessValidator {
    fn new() -> Self {
        UniquenessValidator {}
    }

    fn is_valid(&self, phrase: &Vec<&str>) -> bool {
        let len = &phrase.len();
        let mut set: HashSet<&str> = HashSet::with_capacity(*len);
        set.extend(phrase);

        set.len() == *len
    }
}

pub struct AnagramValidator {}

impl Validator for AnagramValidator {
    fn new() -> Self {
        AnagramValidator {}
    }

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

pub fn solve<T: Validator>(input: &str) -> u32 {
    let phrases = parse(input);
    let validator = T::new();

    phrases.iter()
        .map(|phrase| validator.is_valid(phrase))
        .filter(|&valid| valid)
        .collect::<Vec<bool>>()
        .len() as u32
}

#[cfg(test)]
mod tests {
    use super::{solve, AnagramValidator, UniquenessValidator};

    #[test]
    fn test_cases_star_one() {
        let input = "
            aa bb cc dd ee
            aa bb cc dd aa
            aa bb cc dd aaa
            ";
        assert_eq!(solve::<UniquenessValidator>(input), 2);
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
        assert_eq!(solve::<AnagramValidator>(input), 3);
    }
}
