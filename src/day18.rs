use std::collections::HashMap;

type Register = char;

#[derive(Clone, Debug)]
enum Destination {
    Register(Register),
    Value(i64),
}

impl Destination {
    fn parse(input: &str) -> Result<Self, String> {
        if let Ok(value) = input.parse() {
            Ok(Destination::Value(value))
        } else {
            if input.len() == 1 {
                Ok(Destination::Register(input.chars().take(1).next().unwrap()))
            } else {
                Err(format!("Invalid register {}", input))
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Snd(Destination),
    Rcv(Destination),
    Set(Register, Destination),
    Add(Register, Destination),
    Mul(Register, Destination),
    Mod(Register, Destination),
    Jump(Destination, Destination),
}

impl Instruction {
    fn parse(input: &str) -> Result<Instruction, String> {
        let cleaned = input.trim().to_lowercase();
        let parts = cleaned.split_whitespace().collect::<Vec<_>>();

        match parts[0] {
            "set" | "mul" | "add" | "mod" => {
                let args = &parts[1..];

                if args.len() != 2 {
                    return Err(format!(
                        "Expected exactly two arguments for {} instruction. Found {} in {}",
                        parts[0],
                        args.len(),
                        input
                    ));
                }

                if args[0].len() != 1 {
                    return Err(format!("Invalid register {}", input));
                }
                let register = args[0].to_owned().chars().take(1).next().unwrap();

                match Destination::parse(args[1]) {
                    Ok(destination) => match parts[0] {
                        "set" => Ok(Instruction::Set(register, destination)),
                        "mul" => Ok(Instruction::Mul(register, destination)),
                        "add" => Ok(Instruction::Add(register, destination)),
                        "mod" => Ok(Instruction::Mod(register, destination)),
                        &_ => panic!("Shouldn't be here"), // Famous last words
                    },
                    Err(e) => Err(e),
                }
            }
            "rcv" | "snd" => {
                let args = &parts[1..];

                match Destination::parse(args[0]) {
                    Ok(destination) => match parts[0] {
                        "snd" => Ok(Instruction::Snd(destination)),
                        "rcv" => Ok(Instruction::Rcv(destination)),
                        &_ => panic!("Shouldn't be here"), // Famous last words
                    },
                    Err(e) => Err(e),
                }
            }
            "jgz" => {
                let args = &parts[1..];
                if args.len() != 2 {
                    return Err(format!(
                        "Expected exactly two arguments for {} instruction. Found {} in {}",
                        parts[0],
                        args.len(),
                        input
                    ));
                }

                match (Destination::parse(args[0]), Destination::parse(args[1])) {
                    (Ok(d1), Ok(d2)) => Ok(Instruction::Jump(d1, d2)),
                    _ => Err(format!("Invalid jgz instruction {}", input)),
                }
            }
            &_ => Err(format!("Invalid instruction {}", input)),
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
    ip: usize,
    registers: HashMap<Register, i64>,
    last_played_frequency: Option<i64>,
    last_recovered_frequency: Option<i64>,
}

impl Program {
    fn new(source: &str) -> Result<Program, String> {
        let instructions = source
            .trim()
            .lines()
            .map(|line| Instruction::parse(line))
            .collect::<Vec<_>>();
        if let Some(Err(error)) = instructions
            .clone()
            .into_iter()
            .find(|instruction| instruction.is_err())
        {
            return Err(format!(
                "Failed to build program due to source code error: {}",
                error
            ));
        }

        Ok(Program {
            instructions: instructions.into_iter().map(|i| i.unwrap()).collect(),
            ip: 0,
            registers: HashMap::new(),
            last_played_frequency: None,
            last_recovered_frequency: None,
        })
    }

    fn run(&mut self) {
        while self.last_recovered_frequency.is_none() {
            let current_instruction = self.instructions[self.ip].clone();
            let mut ip_offset = 1;
            // println!(
            //     "Current instruction {:?} at ip {}",
            //     current_instruction, self.ip
            // );

            match current_instruction {
                Instruction::Set(r, v) => self.set_register(&r, &v),
                Instruction::Add(r, v) => {
                    let new_value = Destination::Value(self.get_register(&r) + self.get_value(&v));
                    self.set_register(&r, &new_value);
                }
                Instruction::Mul(r, v) => {
                    let new_value = Destination::Value(self.get_register(&r) * self.get_value(&v));
                    self.set_register(&r, &new_value);
                }
                Instruction::Mod(r, v) => {
                    let new_value = Destination::Value(self.get_register(&r) % self.get_value(&v));
                    self.set_register(&r, &new_value);
                }
                Instruction::Jump(condition, offset) => {
                    if self.get_value(&condition) > 0 {
                        ip_offset = self.get_value(&offset);
                    }
                }
                Instruction::Snd(v) => self.last_played_frequency = Some(self.get_value(&v)),
                Instruction::Rcv(condition) => {
                    if self.get_value(&condition) != 0 {
                        self.last_recovered_frequency = self.last_played_frequency;
                    }
                }
            }
            let new_ip: i64 = (self.ip as i64) + ip_offset;
            assert!(
                new_ip >= 0 && (new_ip as usize) < self.instructions.len(),
                "Invalid instruction pointer"
            );

            self.ip = (new_ip as usize);
        }
    }

    fn get_register(&self, register: &Register) -> i64 {
        *self.registers.get(register).unwrap_or(&0)
    }

    fn set_register(&mut self, register: &Register, value: &Destination) {
        let v = self.get_value(value);
        self.registers.insert(register.clone(), v);
    }

    fn get_value(&self, value: &Destination) -> i64 {
        match value {
            &Destination::Value(v) => v,
            &Destination::Register(r) => *self.registers.get(&r).unwrap_or(&0),
        }
    }
}

pub fn solve(input: &str) -> i64 {
    let mut program = match Program::new(input) {
        Ok(program) => program,
        Err(error) => panic!(error),
    };

    program.run();

    program
        .last_recovered_frequency
        .expect("rcv should been called at least once")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_cases_star_one() {
        let input = "
                set a 1
                add a 2
                mul a a
                mod a 5
                snd a
                set a 0
                rcv a
                jgz a -1
                set a 1
                jgz a -2
                ";

        assert_eq!(solve(input), 4);
    }

    #[test]
    fn test_cases_star_two() {}
}
