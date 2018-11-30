use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn from_string(input: String) -> Self {
        let numbers: Vec<i64> = input
            .replace("<", "")
            .replace(">", "")
            .split(",")
            .map(|value| value.parse::<i64>().expect("Invalid numer"))
            .collect();

        assert!(
            numbers.len() == 3,
            "Expected exactly three numbers per vector got {} for {}",
            numbers.len(),
            input
        );

        Self::new(numbers[0], numbers[1], numbers[2])
    }

    fn manhattan_distance_to_origin(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

#[derive(Debug)]
struct Particle {
    pub position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    fn new(position: Vector, velocity: Vector, acceleration: Vector) -> Self {
        Self {
            position,
            velocity,
            acceleration,
        }
    }

    fn tick(&mut self) {
        self.velocity = self.velocity + self.acceleration;
        self.position = self.position + self.velocity;
    }
}

fn parse(input: &str) -> Vec<Particle> {
    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            let values = line
                .split(", ")
                .map(|definition| {
                    let parts: Vec<&str> = definition.split("=").collect();

                    assert!(
                        parts.len() == 2,
                        "Invalid particle definition {}",
                        definition
                    );

                    Vector::from_string(parts[1].to_owned())
                })
                .collect::<Vec<Vector>>();

            assert!(values.len() == 3, "Each particle should have 3 values.");

            Particle::new(values[0], values[1], values[2])
        })
        .collect()
}

pub fn star_one(input: &str) -> usize {
    let mut particles = parse(input);

    // 1000 iterations is just an arbitrary choice
    for _ in 1..1000 {
        for particle in &mut particles {
            particle.tick();
        }
    }

    let mut distances = particles
        .iter()
        .enumerate()
        .map(|(id, p)| (id, p.position.manhattan_distance_to_origin() as i64))
        .collect::<Vec<(usize, i64)>>();

    distances.sort_by(|a, b| a.1.cmp(&b.1));

    distances[0].0
}

pub fn star_two(input: &str) -> usize {
    let mut particles: Vec<Option<Particle>> = parse(input).into_iter().map(|p| Some(p)).collect();

    // 1000 iterations is just an arbitrary choice
    for _ in 1..1000 {
        for particle in &mut particles {
            particle.as_mut().map(|ref mut p| p.tick());
        }

        let mut particles_by_distance_to_origin = HashMap::<i64, Vec<usize>>::new();
        particles
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_some())
            .for_each(|(id, p)| {
                particles_by_distance_to_origin
                    .entry(p.as_ref().unwrap().position.manhattan_distance_to_origin())
                    .or_insert(Vec::<usize>::new())
                    .push(id);
            });

        particles_by_distance_to_origin.iter().for_each(|(_, ids)| {
            if ids.len() > 1 {
                let mut confirmed_collisions = HashSet::new();

                for id1 in ids {
                    for id2 in ids {
                        if id1 == id2 {
                            continue;
                        }

                        let p1 = &particles[*id1];
                        let p2 = &particles[*id2];
                        let mut is_equal = p1
                            .as_ref()
                            .map(|x| {
                                p2.as_ref()
                                    .map(|y| x.position == y.position)
                                    .unwrap_or(false)
                            })
                            .unwrap_or(false);

                        if is_equal {
                            confirmed_collisions.insert(id1);
                            confirmed_collisions.insert(id2);
                        }
                    }
                }

                for id in confirmed_collisions {
                    particles[*id] = None;
                }
            }
        });
    }

    particles.iter().filter(|x| x.is_some()).count()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(""), 1)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
