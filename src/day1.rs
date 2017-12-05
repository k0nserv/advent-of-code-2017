fn shift<T: Copy>(vec: &Vec<T>, distance: usize) -> Vec<T> {
    let mut new_vector = Vec::with_capacity(vec.len());

    for i in distance..vec.len() + distance {
        if i >= vec.len() {
            new_vector.push(vec[i % vec.len()]);
        } else {
            new_vector.push(vec[i]);
        }
    }

    new_vector
}

pub fn solve(input: &str, distance: usize) -> u32 {
    let parsed = input
        .trim()
        .chars()
        .map(|v| v.to_digit(10).expect("Expected digits"))
        .collect::<Vec<u32>>();
    let shifted = shift(&parsed, distance);
    let zipped = parsed.iter().zip(shifted.iter());

    zipped.fold(0, |acc, (x, y)| if x == y { acc + x } else { acc })
}

#[cfg(test)]
mod tests {
    use super::{shift, solve};

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve("1122", 1), 3);
        assert_eq!(solve("1111", 1), 4);
        assert_eq!(solve("1234", 1), 0);
        assert_eq!(solve("91212129", 1), 9);
    }

    #[test]
    fn test_cases_star_two() {
        assert_eq!(solve("1212", 2), 6);
        assert_eq!(solve("1221", 2), 0);
        assert_eq!(solve("123425", 3), 4);
        assert_eq!(solve("123123", 3), 12);
        assert_eq!(solve("12131415", 4), 4);
    }

    #[test]
    fn test_shift() {
        let vec = vec![1, 2, 3, 4];
        assert_eq!(shift(&vec, 1), vec![2, 3, 4, 1]);
    }
}
