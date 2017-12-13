pub fn solve<F>(input: &str, alter: F) -> u32
where
    F: Fn(i32) -> i32,
{
    let mut jumps: Vec<i32> = input
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    // Instruction pointer
    let mut ip: i32 = 0;
    let mut steps = 0;
    let len = jumps.len() as i32;

    while ip >= 0 && ip < len {
        let instruction = jumps[ip as usize];
        jumps[ip as usize] += alter(instruction);
        ip += instruction;
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve("0 3 0 1 -3", |_| 1), 5);
    }

    #[test]
    fn test_cases_star_two() {
        assert_eq!(solve("0 3 0 1 -3", |i| if i >= 3 { -1 } else { 1 }), 10);
    }
}
