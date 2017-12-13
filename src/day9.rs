#[derive(Debug)]
struct State {
    depth: i32,
    score: i32,
    ignore: bool,
    in_garbage: bool,
    garbage_count: i32,
}

impl State {
    fn new() -> Self {
        State {
            depth: 0,
            score: 0,
            ignore: false,
            in_garbage: false,
            garbage_count: 0,
        }
    }
}

fn change(score: i32, in_garbage: bool, amount: i32) -> i32 {
    if in_garbage {
        return score;
    }

    score + amount
}

pub fn solve(input: &str) -> (i32, i32) {
    let mut state = State::new();

    input.chars().for_each(|token| {
        let mut should_reset_ignore = state.ignore;

        if !state.ignore && state.in_garbage && token != '>' && token != '!' {
            state.garbage_count += 1;
        }

        match token {
            '{' => state.depth = change(state.depth, state.in_garbage, 1),
            '}' => {
                state.score = change(state.score, state.in_garbage, state.depth);
                state.depth = change(state.depth, state.in_garbage, -1);
            }
            '<' => {
                if !state.ignore && !state.in_garbage {
                    state.in_garbage = true;
                }
            }
            '>' => {
                if !state.ignore && state.in_garbage {
                    state.in_garbage = false
                }
            }
            '!' => {
                if !state.ignore {
                    state.ignore = true;
                    should_reset_ignore = false;
                }
            }
            ',' => {}
            _ => {}
        }

        if should_reset_ignore {
            state.ignore = false;
        }
    });

    (state.score, state.garbage_count)
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve("{}"), (1, 0));
        assert_eq!(solve("{{{}}}"), (6, 0));
        assert_eq!(solve("{{},{}}"), (5, 0));
        assert_eq!(solve("{{{},{},{{}}}}"), (16, 0));
        assert_eq!(solve("{<a>,<a>,<a>,<a>}"), (1, 4));
        assert_eq!(solve("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
        assert_eq!(solve("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
        assert_eq!(solve("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
    }
}
