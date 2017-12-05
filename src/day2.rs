fn permutate(values: &Vec<i32>) -> Vec<[i32; 2]> {
    values
        .iter()
        .enumerate()
        .flat_map(move |(i, &v)| {
            values
                .iter()
                .enumerate()
                .map(move |(ii, &vv)| if i == ii { None } else { Some([v, vv]) })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
        })
        .collect()
}

pub fn row_data_min_max(row: &Vec<i32>) -> i32 {
    row.iter().max().unwrap() - row.iter().min().unwrap()
}

pub fn row_data_evenly_divisible(row: &Vec<i32>) -> i32 {
    permutate(row)
        .iter()
        .filter(|values| values[0] % values[1] == 0)
        .map(|v| v[0] / v[1])
        .sum()
}

pub fn solve(input: &str, data_for_row: &Fn(&Vec<i32>) -> i32) -> i32 {
    input
        .split("\n")
        .map(|row| {
            row.trim()
                .split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|row| !row.is_empty())
        .map(|row| data_for_row(&row))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{row_data_evenly_divisible, row_data_min_max, solve};

    #[test]
    fn test_cases_star_one() {
        assert_eq!(
            solve(
                "
            5 1 9 5
            7 5 3
            2 4 6 8
            ",
                &row_data_min_max
            ),
            18
        );
    }

    #[test]
    fn test_cases_star_two() {
        assert_eq!(
            solve(
                "
            5 9 2 8
            9 4 7 3
            3 8 6 5
            ",
                &row_data_evenly_divisible
            ),
            9
        );
    }

    #[test]
    fn test_row_data_evenly_divisible() {
        assert_eq!(row_data_evenly_divisible(&vec![5, 9, 2, 8]), 4);
        assert_eq!(row_data_evenly_divisible(&vec![9, 4, 7, 3]), 3);
        assert_eq!(row_data_evenly_divisible(&vec![3, 8, 6, 5]), 2);
    }
}
