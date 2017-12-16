#![feature(i128_type)] // For day 14

use std::time::{SystemTime, UNIX_EPOCH};

mod day1;
mod day2;
mod day3;
mod grid;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn time<F>(closure: F)
where
    F: Fn(),
{
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    closure();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = end - start;
    println!(
        "Time taken: {}s and {}ns",
        time.as_secs(),
        time.subsec_nanos()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(solve(&input, |i| if i >= 3 { -1 } else { 1 }), 29227751);
    }

    #[test]
    fn solve_day6() {
        use day6::solve;

        let input = load_file("day6.txt");

        assert_eq!(solve(&input), (11137, 1037));
    }

    #[test]
    fn solve_day7() {
        use day7::solve;

        let input = load_file("day7.txt");

        assert_eq!(solve(&input), ("qibuqqg".to_owned(), 1079));
    }

    #[test]
    fn solve_day8() {
        use day8::solve;

        let input = load_file("day8.txt");

        assert_eq!(solve(&input), (2971, 4254));
    }

    #[test]
    fn solve_day9() {
        use day9::solve;

        let input = load_file("day9.txt");

        assert_eq!(solve(&input), (17390, 7825));
    }

    #[test]
    fn solve_day10() {
        use day10::{solve, solve2};

        let input = "106,16,254,226,55,2,1,166,177,247,93,0,255,228,60,36";

        assert_eq!(solve(&input, 256), 11413);
        assert_eq!(solve2(&input, 256, 64), "7adfd64c2a03a4968cf708d1b7fd418d");
    }

    #[test]
    fn solve_day11() {
        use day11::{solve, solve2};

        let input = load_file("day11.txt");

        assert_eq!(solve(&input), 675);
        assert_eq!(solve2(&input), 1424);
    }

    #[test]
    fn solve_day12() {
        use day12::{solve, solve2};

        let input = load_file("day12.txt");

        assert_eq!(solve(&input), 134);
        assert_eq!(solve2(&input), 134);
    }

    #[test]
    fn solve_day13() {
        use day13::{solve, solve2};

        let input = load_file("day13.txt");

        assert_eq!(solve(&input), 1476);
        assert_eq!(solve2(&input), 3937334);
    }

    #[test]
    fn solve_day14() {
        use day14::{solve, solve2};

        let input = "hwlqcszp";

        assert_eq!(solve(&input), 8304);
        assert_eq!(solve2(&input), 1018);
    }

    #[test]
    fn solve_day15() {
        use day15::{solve, solve2};

        time(|| assert_eq!(solve(783, 325), 650));
        time(|| assert_eq!(solve2(783, 325), 336));
    }

    #[test]
    fn solve_day16() {
        use day16::solve;

        let input = load_file("day16.txt");

        assert_eq!(solve(&input, 16, 1), "fgmobeaijhdpkcln");

        // Cycles at 24 iteratiors. 1_000_000_000 % 24 == 16
        assert_eq!(solve(&input, 16, 16), "fgmobeaijhdpkcln");
    }
}
