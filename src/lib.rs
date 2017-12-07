mod day1;
mod day2;
mod day3;
mod grid;
mod day4;
mod day5;
mod day6;

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
        use day4::{solve, AnagramValidator, UniquenessValidator};


        let input = load_file("day4.txt");

        assert_eq!(solve::<UniquenessValidator>(&input), 451);
        assert_eq!(solve::<AnagramValidator>(&input), 223);
    }

    #[test]
    fn solve_day5() {
        use day5::solve;

        let input = load_file("day5.txt");

        assert_eq!(solve(&input, |_| 1), 376976);
        assert_eq!(
            solve(&input, |i| {
                if i >= 3 {
                    -1
                } else {
                    1
                }
            }),
            29227751
        );
    }

    #[test]
    fn solve_day6() {
        use day6::solve;
        use std::time::{SystemTime, UNIX_EPOCH};

        let input = load_file("day6.txt");

        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        assert_eq!(solve(&input), (11137, 1037));
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let time = end - start;
        println!("Time taken: {}s and {}ns", time.as_secs(), time.subsec_nanos());
    }
}
