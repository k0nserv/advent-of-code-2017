use std::collections::HashMap;

enum Operator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

impl Operator {
    fn parse(token: &str) -> Option<Operator> {
        match token {
            ">" => Some(Operator::GreaterThan),
            ">=" => Some(Operator::GreaterThanOrEqual),
            "<" => Some(Operator::LessThan),
            "<=" => Some(Operator::LessThanOrEqual),
            "==" => Some(Operator::Equal),
            "!=" => Some(Operator::NotEqual),
            _ => None,
        }
    }

    fn evaluate(&self, operator_value: i32, test_value: i32) -> bool {
        match self {
            &Operator::Equal => operator_value == test_value,
            &Operator::NotEqual => operator_value != test_value,
            &Operator::LessThan => test_value < operator_value,
            &Operator::LessThanOrEqual => test_value <= operator_value,
            &Operator::GreaterThan => test_value > operator_value,
            &Operator::GreaterThanOrEqual => test_value >= operator_value,
        }
    }
}

struct Condition {
    target_register: String,
    operator: Operator,
    value: i32,
}

impl Condition {
    fn new(target_register: String, operator: Operator, value: i32) -> Self {
        Condition {
            target_register: target_register,
            operator: operator,
            value: value,
        }
    }

    fn parse(tokens: &[&str]) -> Option<Condition> {
        assert!(
            tokens.len() == 3,
            "There should be exactly three condition tokens"
        );
        if tokens.len() != 3 {
            return None;
        }

        let target_register = tokens[0];
        let operator = Operator::parse(tokens[1]).expect(&format!("Bad operator {}", tokens[1]));
        let value = tokens[2]
            .parse::<i32>()
            .expect(&format!("Bad condition value {}", tokens[2]));

        Some(Condition::new(target_register.to_owned(), operator, value))
    }

    fn evaluate(&self, test_value: i32) -> bool {
        self.operator.evaluate(self.value, test_value)
    }
}

struct Expression<'a> {
    target_register: &'a str,
    operation: Operation,
    condition: Condition,
}

impl<'a> Expression<'a> {
    fn new(target_register: &'a str, operation: Operation, condition: Condition) -> Expression<'a> {
        Expression {
            target_register: target_register,
            operation: operation,
            condition: condition,
        }
    }
}

enum Operation {
    Inc(i32),
    Dec(i32),
}

impl Operation {
    fn parse(tokens: &[&str]) -> Option<Operation> {
        assert!(
            tokens.len() == 2,
            "There should be exactly two operation tokens"
        );
        if tokens.len() != 2 {
            return None;
        }

        let value = tokens[1].parse::<i32>();
        if value.is_err() {
            return None;
        }

        match tokens[0].trim().to_lowercase().as_ref() {
            "inc" => Some(Operation::Inc(value.unwrap())),
            "dec" => Some(Operation::Dec(value.unwrap())),
            _ => None,
        }
    }
}

impl Operation {
    fn value(&self) -> i32 {
        match *self {
            Operation::Inc(v) => v,
            Operation::Dec(v) => -v,
        }
    }
}

struct Registers<'a> {
    registers: HashMap<&'a str, i32>,
    largest_observed_value: i32,
}

impl<'a> Registers<'a> {
    fn new() -> Self {
        Registers {
            registers: HashMap::<_, _>::new(),
            largest_observed_value: 0,
        }
    }

    fn max_register_value(&self) -> i32 {
        *self.registers.values().max().unwrap_or(&0)
    }

    fn get(&self, register: &str) -> i32 {
        *self.registers.get(register).unwrap_or(&0)
    }

    fn apply(&mut self, register: &'a str, op: &Operation) {
        let value = self.get(register);
        let new_value = value + op.value();
        self.registers.insert(register, new_value);

        if new_value > self.largest_observed_value {
            self.largest_observed_value = new_value;
        }
    }

    fn evaluate(&mut self, expression: &'a Expression) {
        let condition_value = self.get(expression.condition.target_register.as_ref());
        if !expression.condition.evaluate(condition_value) {
            return;
        }

        self.apply(expression.target_register, &expression.operation);
    }
}

fn parse(input: &str) -> Vec<Box<Expression>> {
    input
        .trim()
        .lines()
        .map(|row| {
            let tokens = row.split_whitespace().collect::<Vec<_>>();
            assert!(
                tokens.len() == 7,
                format!(
                    "Expected exactly 7 tokens per expression. Found {} in {:?}",
                    tokens.len(),
                    tokens
                )
            );
            let target_register = tokens[0];
            let operation = Operation::parse(&tokens[1..3]).unwrap();
            let condition = Condition::parse(&tokens[4..7]).unwrap();

            Box::new(Expression::new(target_register, operation, condition))
        }).collect()
}

pub fn solve(input: &str) -> (i32, i32) {
    let program = parse(input);
    let mut registers = Registers::new();

    for expression in program.iter() {
        registers.evaluate(expression);
    }

    (
        registers.max_register_value(),
        registers.largest_observed_value,
    )
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_cases_star_one() {
        assert_eq!(
            solve(
                "
        b inc 5 if a > 1
        a inc 1 if b < 5
        c dec -10 if a >= 1
        c inc -20 if c == 10
            "
            ),
            (1, 10)
        );
    }

}
