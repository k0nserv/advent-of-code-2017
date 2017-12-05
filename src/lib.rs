mod day1;
mod day2;
mod day3;
mod grid;
mod day4;
mod day5;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    fn load_file(path: &str) -> String {
        let mut input = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut input).expect("Unable to read string");

        input
    }


    #[test]
    fn solve_day1() {
        use day1::solve;
        let input = load_file("day1.txt");

        assert_eq!(solve(&input, 1), 1343);
        assert_eq!(solve(&input, input.len() / 2), 1274);
    }

    #[test]
    fn solve_day2() {
        use day2::{row_data_evenly_divisible, row_data_min_max, solve};
        let input = load_file("day2.txt");

        assert_eq!(solve(&input, &row_data_min_max), 53460);
        assert_eq!(solve(&input, &row_data_evenly_divisible), 282);
    }

    #[test]
    fn solve_day3() {
        use day3::{solve, solve_star_two};

        assert_eq!(solve(312051), 430);
        assert_eq!(solve_star_two(312051), 312453);
    }

    #[test]
    fn solve_day4() {
        use std::fs::File;
        use std::io::Read;

        use day4::{solve, UniquenessValidator, AnagramValidator};

        let mut input = String::new();
        let mut f = File::open("day4.txt").expect("Unable to open file");
        f.read_to_string(&mut input).expect("Unable to read string");

        assert_eq!(solve::<UniquenessValidator>(&input), 451);
        assert_eq!(solve::<AnagramValidator>(&input), 223);
    }
}
