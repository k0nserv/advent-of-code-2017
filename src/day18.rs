use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

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

type Registers = HashMap<Register, i64>;

fn get_register(registers: &Registers, register: &Register) -> i64 {
    *registers.get(register).unwrap_or(&0)
}

fn set_register(registers: &mut Registers, register: &Register, value: &Destination) {
    let v = get_value(registers, value);
    registers.insert(register.clone(), v);
}

fn get_value(registers: &Registers, value: &Destination) -> i64 {
    match value {
        &Destination::Value(v) => v,
        &Destination::Register(r) => *registers.get(&r).unwrap_or(&0),
    }
}

struct Program2 {
    instructions: Vec<Instruction>,
    ip: usize,
    registers: HashMap<Register, i64>,
    id: usize,
    queue: VecDeque<i64>,
    sibling_program: Option<Rc<RefCell<Program2>>>,
    send_counter: usize,
}

impl Program2 {
    fn new(source: &str, id: usize) -> Result<Self, String> {
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
        let mut hash_map = HashMap::new();
        hash_map.insert('p', id as i64);

        Ok(Self {
            id,
            instructions: instructions.into_iter().map(|i| i.unwrap()).collect(),
            ip: 0,
            registers: hash_map,
            queue: VecDeque::new(),
            sibling_program: None,
            send_counter: 0,
        })
    }

    fn current_instruction(&self) -> Instruction {
        self.instructions[self.ip].clone()
    }

    fn set_sibling(&mut self, sibling: Rc<RefCell<Program2>>) {
        self.sibling_program = Some(sibling);
    }

    fn is_deadlocked(&self) -> bool {
        match self.current_instruction() {
            Instruction::Rcv(_) => self.queue.is_empty(),
            _ => false,
        }
    }

    fn add_to_queue(&mut self, value: i64) {
        self.queue.push_back(value);
    }

    fn send(&mut self, destination: Destination) {
        println!("Sending {:?} in program {}", destination, self.id);
        if self.id == 1 {
            self.send_counter += 1;
        }

        match &self.sibling_program {
            Some(p) => p
                .borrow_mut()
                .add_to_queue(get_value(&self.registers, &destination)),
            None => (),
        };
        ()
    }

    fn tick(&mut self) {
        let current_instruction = self.current_instruction();
        let mut ip_offset = 1;
        // println!(
        //     "Current instruction {:?} at ip {} in program {}",
        //     current_instruction, self.ip, self.id
        // );

        match current_instruction {
            Instruction::Set(r, v) => set_register(&mut self.registers, &r, &v),
            Instruction::Add(r, v) => {
                let new_value = Destination::Value(
                    get_register(&mut self.registers, &r) + get_value(&self.registers, &v),
                );
                set_register(&mut self.registers, &r, &new_value);
            }
            Instruction::Mul(r, v) => {
                let new_value = Destination::Value(
                    get_register(&self.registers, &r) * get_value(&self.registers, &v),
                );
                set_register(&mut self.registers, &r, &new_value);
            }
            Instruction::Mod(r, v) => {
                let new_value = Destination::Value(
                    get_register(&self.registers, &r) % get_value(&self.registers, &v),
                );
                set_register(&mut self.registers, &r, &new_value);
            }
            Instruction::Jump(condition, offset) => {
                if get_value(&self.registers, &condition) > 0 {
                    ip_offset = get_value(&self.registers, &offset);
                }
            }
            Instruction::Snd(v) => (self.send(v)),
            Instruction::Rcv(register) => match register {
                Destination::Register(r) => {
                    if self.queue.is_empty() {
                        ip_offset = 0;
                    } else {
                        let value = self.queue.pop_front().unwrap();
                        set_register(&mut self.registers, &r, &Destination::Value(value));
                    }
                }
                Destination::Value(_) => {
                    assert!(false, "Shouldn't be here");
                }
            },
        }
        let new_ip: i64 = (self.ip as i64) + ip_offset;
        assert!(
            new_ip >= 0 && (new_ip as usize) < self.instructions.len(),
            "Invalid instruction pointer"
        );

        self.ip = (new_ip as usize);
    }
}

struct Program1 {
    instructions: Vec<Instruction>,
    ip: usize,
    registers: HashMap<Register, i64>,
    last_played_frequency: Option<i64>,
    last_recovered_frequency: Option<i64>,
}

impl Program1 {
    fn new(source: &str) -> Result<Self, String> {
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

        Ok(Self {
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
                Instruction::Set(r, v) => set_register(&mut self.registers, &r, &v),
                Instruction::Add(r, v) => {
                    let new_value = Destination::Value(
                        get_register(&mut self.registers, &r) + get_value(&self.registers, &v),
                    );
                    set_register(&mut self.registers, &r, &new_value);
                }
                Instruction::Mul(r, v) => {
                    let new_value = Destination::Value(
                        get_register(&self.registers, &r) * get_value(&self.registers, &v),
                    );
                    set_register(&mut self.registers, &r, &new_value);
                }
                Instruction::Mod(r, v) => {
                    let new_value = Destination::Value(
                        get_register(&self.registers, &r) % get_value(&self.registers, &v),
                    );
                    set_register(&mut self.registers, &r, &new_value);
                }
                Instruction::Jump(condition, offset) => {
                    if get_value(&self.registers, &condition) > 0 {
                        ip_offset = get_value(&self.registers, &offset);
                    }
                }
                Instruction::Snd(v) => {
                    self.last_played_frequency = Some(get_value(&self.registers, &v))
                }
                Instruction::Rcv(condition) => {
                    if get_value(&self.registers, &condition) != 0 {
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
}

pub fn solve(input: &str) -> i64 {
    let mut program = match Program1::new(input) {
        Ok(program) => program,
        Err(error) => panic!(error),
    };

    program.run();

    program
        .last_recovered_frequency
        .expect("rcv should been called at least once")
}

pub fn solve2(input: &str) -> usize {
    let mut program1 = match Program2::new(input, 0) {
        Ok(program) => Rc::new(RefCell::new(program)),
        Err(error) => panic!(error),
    };

    let mut program2 = match Program2::new(input, 1) {
        Ok(program) => Rc::new(RefCell::new(program)),
        Err(error) => panic!(error),
    };

    program1.borrow_mut().set_sibling(program2.clone());
    program2.borrow_mut().set_sibling(program1.clone());

    while !program1.borrow().is_deadlocked() || !program2.borrow().is_deadlocked() {
        program1.borrow_mut().tick();
        program2.borrow_mut().tick();
    }

    let result = program2.borrow().send_counter;

    result
}

#[cfg(test)]
mod tests {
    use super::{solve, solve2};

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
    fn test_cases_star_two() {
        let input = "
                snd 1
                snd 2
                snd p
                rcv a
                rcv b
                rcv c
                rcv d
                ";
        assert_eq!(solve2(input), 3);
    }
}
