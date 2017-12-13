#[derive(Debug)]
enum Direction {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
}

impl Direction {
    fn parse(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_ref() {
            "n" => Some(Direction::N),
            "ne" => Some(Direction::NE),
            "nw" => Some(Direction::NW),
            "s" => Some(Direction::S),
            "se" => Some(Direction::SE),
            "sw" => Some(Direction::SW),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Location {
    x: i32,
    y: i32,
    z: i32,
}

impl Location {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Location { x, y, z }
    }

    fn origin() -> Self {
        Self::new(0, 0, 0)
    }

    fn mv(self, direction: &Direction) -> Location {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        match direction {
            &Direction::N => Location::new(x, y + 1, z - 1),
            &Direction::NE => Location::new(x + 1, y, z - 1),
            &Direction::NW => Location::new(x - 1, y + 1, z),
            &Direction::S => Location::new(x, y - 1, z + 1),
            &Direction::SE => Location::new(x + 1, y - 1, z),
            &Direction::SW => Location::new(x - 1, y, z + 1),
        }
    }

    fn manhattan_distance(&self, other: &Location) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }
}

pub fn solve(input: &str) -> i32 {
    let steps = input
        .trim()
        .split(',')
        .map(|s| Direction::parse(s.trim()).expect(&format!("Unparsable direction {}", s)))
        .collect::<Vec<_>>();

    let loc =
        steps
            .iter()
            .fold(Location::origin(), |acc, step| acc.mv(step));

    loc.manhattan_distance(&Location::origin())
}

pub fn solve2(input: &str) -> i32 {
    let steps = input
        .trim()
        .split(',')
        .map(|s| Direction::parse(s.trim()).expect(&format!("Unparsable direction {}", s)))
        .collect::<Vec<_>>();

    let origin = Location::origin();
    steps
        .iter()
        .fold((Location::origin(), 0), |(acc, max_distance), step| {
            let distance = origin.manhattan_distance(&acc);
            let new_max = if distance > max_distance {
                distance
            } else {
                max_distance
            };

            (acc.mv(step), new_max)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve("ne,ne,ne"), 3);
        assert_eq!(solve("ne,ne,sw,sw"), 0);
        assert_eq!(solve("ne,ne,s,s"), 2);
        assert_eq!(solve("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn test_mv() {
        assert_eq!(
            Location::origin().mv(&Direction::N),
            Location::new(0, 1, -1)
        );
        assert_eq!(
            Location::origin().mv(&Direction::NE),
            Location::new(1, 0, -1)
        );
        assert_eq!(
            Location::origin().mv(&Direction::NW),
            Location::new(-1, 1, 0)
        );
        assert_eq!(
            Location::origin().mv(&Direction::S),
            Location::new(0, -1, 1)
        );
        assert_eq!(
            Location::origin().mv(&Direction::SE),
            Location::new(1, -1, 0)
        );
        assert_eq!(
            Location::origin().mv(&Direction::SW),
            Location::new(-1, 0, 1)
        );
    }
}
