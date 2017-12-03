use std::ops::Add;
use grid::Grid;
use std::iter;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }

    fn manhattan_distance(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

type GridDefinition = (u32, u32, u32);

fn determine_min_grid_size(iloc: u32) -> GridDefinition {
    let mut current = 1;
    let mut level = 0;

    while current * current < iloc {
        current = current + 2;
        level += 1;
    }

    (current * current, current, level)
}

fn find_point(iloc: u32, grid: GridDefinition) -> Point {
    let grid_side = grid.1;
    if grid.2 == 0 {
        return Point::new(0, 0);
    }

    let mut location = Point::new(grid.2 as i32, -(grid.2 as i32));
    let mut current_iloc = grid_side * grid_side;

    let steps = [(grid_side - 1, Point::new(-1, 0)),
                 (grid_side - 1, Point::new(0, 1)),
                 (grid_side - 1, Point::new(1, 0)),
                 (grid_side - 1, Point::new(0, -1))];

    for step in steps.iter() {
        for _ in 0..step.0 {
            if iloc == current_iloc {
                return location;
            }
            current_iloc -= 1;
            location = location + step.1;
        }
    }


    assert!(false, "Should not be here");
    Point::new(0, 0)
}


pub fn solve(iloc: u32) -> u32 {
    let origin = Point::new(0, 0);
    let grid = determine_min_grid_size(iloc);
    let loc = find_point(iloc, grid);


    loc.manhattan_distance(&origin)
}

pub fn solve_star_two(target: u32) -> u32 {
    let min_grid = determine_min_grid_size(target);
    println!("Min grid: {:?}", min_grid);
    let mut grid: Grid = Grid::new(min_grid.1 as usize);
    grid[(0, 0)] = Some(1);

    let mut loc: (i32, i32) = (0, 0);
    let mut current_side_length = 3;
    let mut up_steps = 1;

    let up: (i32, i32) = (0, 1);
    let left: (i32, i32) = (-1, 0);
    let down = (0, -1);
    let right = (1, 0);

    loop {
        let mut steps: Vec<(i32, i32)> = Vec::new();
        steps.extend(iter::repeat(right).take(1));
        steps.extend(iter::repeat(up).take(up_steps));
        steps.extend(iter::repeat(left).take(current_side_length - 1));
        steps.extend(iter::repeat(down).take(current_side_length - 1));
        steps.extend(iter::repeat(right).take(current_side_length - 1));

        for step in steps.iter() {
            loc = (loc.0 + step.0, loc.1 + step.1);
            let value = grid.sum_of_neighbours(loc);
            if value > target {
                return value;
            }
            grid[loc] = Some(value);
        }

        current_side_length += 2;
        up_steps += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, solve, solve_star_two, determine_min_grid_size, find_point};

    #[test]
    fn test_manhattan_distance() {
        let cases = [(Point::new(0, 0), Point::new(0, 0), 0),
                     (Point::new(0, 0), Point::new(2, 1), 3),
                     (Point::new(0, 0), Point::new(0, -2), 2)];

        for case in cases.iter() {
            assert_eq!(case.0.manhattan_distance(&case.1), case.2);
        }
    }

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve(0), 0);
        assert_eq!(solve(12), 3);
        assert_eq!(solve(23), 2);
        assert_eq!(solve(1024), 31);
    }

    #[test]
    fn test_cases_star_two() {
        assert_eq!(solve_star_two(5), 10);
        assert_eq!(solve_star_two(133), 142);
        assert_eq!(solve_star_two(26), 54);
        assert_eq!(solve_star_two(747), 806);
        assert_eq!(solve_star_two(362), 747);
    }

    #[test]
    fn test_determine_min_grid_size() {
        assert_eq!(determine_min_grid_size(1), (1, 1, 0));
        assert_eq!(determine_min_grid_size(2), (9, 3, 1));
        assert_eq!(determine_min_grid_size(10), (25, 5, 2));
        assert_eq!(determine_min_grid_size(25), (25, 5, 2));
        assert_eq!(determine_min_grid_size(1024), (1089, 33, 16));
        assert_eq!(determine_min_grid_size(312051), (312481, 559, 279));
    }

    #[test]
    fn test_find_point() {
        assert_eq!(find_point(1, (1, 1, 0)), Point::new(0, 0));
        assert_eq!(find_point(3, (9, 3, 1)), Point::new(1, 1));
        assert_eq!(find_point(12, (25, 5, 2)), Point::new(2, 1));
    }
}
