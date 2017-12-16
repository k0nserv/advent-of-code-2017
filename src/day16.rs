use std::collections::HashSet;
use std::fmt;

struct Programs {
    programs: Vec<char>,
}

impl Programs {
    fn new(num_programs: u32) -> Self {
        assert!(num_programs <= 26, "Cannot handle more than 26 letters");

        Programs {
            programs: (0..num_programs)
                .map(|i| (i as u8 + 97 as u8) as char)
                .collect::<Vec<_>>(),
        }
    }

    fn apply(&mut self, action: &Action) {
        match action {
            &Action::Spin(offset) => {
                self.programs = self.programs
                    .clone()
                    .into_iter()
                    .cycle()
                    .skip(self.programs.len() - offset)
                    .take(self.programs.len())
                    .collect()
            }
            &Action::Exchange(i1, i2) => {
                let t1 = self.programs[i1];
                let t2 = self.programs[i2];
                self.programs[i2] = t1;
                self.programs[i1] = t2;
            }
            &Action::Partner(a, b) => {
                let a_index = self.programs.iter().position(|&c| c == a).unwrap();
                let b_index = self.programs.iter().position(|&c| c == b).unwrap();

                self.programs[a_index] = b;
                self.programs[b_index] = a;
            }
        }
    }
}

impl fmt::Display for Programs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.programs.iter().collect::<String>())
    }
}

enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Action {
    fn parse(input: &str) -> Option<Action> {
        let clean: String = input.trim().to_lowercase();
        match &clean[0..1] {
            "s" => {
                let offset = input[1..].parse::<usize>();

                offset.map(|v| Some(Action::Spin(v))).unwrap_or(None)
            }
            "x" => {
                let rest = input[1..].split("/").collect::<Vec<_>>();
                let p1 = rest[0]
                    .trim()
                    .parse::<usize>()
                    .expect("Expected numeric exchange argument");
                let p2 = rest[1]
                    .trim()
                    .parse::<usize>()
                    .expect("Expected numeric exchange argument");

                Some(Action::Exchange(p1, p2))
            }
            "p" => {
                let rest = input[1..].split("/").collect::<Vec<_>>();
                let p1 = rest[0].chars().take(1).collect::<Vec<_>>()[0];
                let p2 = rest[1].chars().take(1).collect::<Vec<_>>()[0];

                Some(Action::Partner(p1, p2))
            }
            _ => None,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Action::Spin(offset) => write!(f, "s{}", offset),
            &Action::Exchange(i1, i2) => write!(f, "x{}/{}", i1, i2),
            &Action::Partner(a, b) => write!(f, "p{}/{}", a, b),
        }
    }
}

pub fn solve(input: &str, num_programs: u32, repeats: u32) -> String {
    let mut programs = Programs::new(num_programs);
    let actions = input
        .trim()
        .split(',')
        .map(|value| Action::parse(value).expect("Invalid action"))
        .collect::<Vec<_>>();

    for i in (0..repeats) {
        for action in actions.iter() {
            programs.apply(action);
        }
    }

    programs.to_string()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve("s1,x3/4,pe/b", 5, 1), "baedc");
    }
}
