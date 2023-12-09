use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone)]
enum Operand {
    Num(u16),
    Ident(String),
}

impl Operand {
    fn parse(s: &str) -> Self {
        s.parse::<u16>()
            .map_or_else(|_| Self::Ident(s.to_string()), Self::Num)
    }
}

#[derive(Clone)]
enum Operation {
    Operand(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
}

impl Operation {
    fn parse(input: &str) -> Self {
        if let Some((op1, op2)) = input.split_once(" AND ") {
            Self::And(Operand::parse(op1), Operand::parse(op2))
        } else if let Some((op1, op2)) = input.split_once(" OR ") {
            Self::Or(Operand::parse(op1), Operand::parse(op2))
        } else if let Some((op1, op2)) = input.split_once(" LSHIFT ") {
            Self::LShift(Operand::parse(op1), Operand::parse(op2))
        } else if let Some((op1, op2)) = input.split_once(" RSHIFT ") {
            Self::RShift(Operand::parse(op1), Operand::parse(op2))
        } else if let Some(op) = input.strip_prefix("NOT ") {
            Self::Not(Operand::parse(op))
        } else {
            Self::Operand(Operand::parse(input))
        }
    }
}

#[derive(Clone)]
struct Instruction {
    input: Operation,
    destination: String,
}

macro_rules! unwrap_or_continue {
    ($e:expr) => {{
        let Some(value) = $e else {
            continue;
        };

        value
    }};
}

fn main() {
    let mut wires = HashMap::<String, u16>::new();

    let instructions = include_str!(r"..\..\input\day7.txt")
        .lines()
        .map(|line| {
            let (input, destination) = line.split_once(" -> ").unwrap();

            let input = Operation::parse(input);

            Instruction {
                input,
                destination: destination.to_string(),
            }
        })
        .collect_vec();

    for _ in 0..200 {
        for instruction in instructions.clone() {
            if wires.get(&instruction.destination).is_none() {
                match instruction.input.clone() {
                    Operation::Operand(operand) => {
                        let value = match operand {
                            Operand::Num(value) => value,
                            Operand::Ident(ident) => *unwrap_or_continue!(wires.get(&ident)),
                        };

                        wires.insert(instruction.destination.clone(), value);
                    }

                    Operation::And(op1, op2) => {
                        let value = match (op1, op2) {
                            (Operand::Num(v1), Operand::Num(v2)) => v1 & v2,
                            (Operand::Num(value), Operand::Ident(ident))
                            | (Operand::Ident(ident), Operand::Num(value)) => {
                                value & *unwrap_or_continue!(wires.get(&ident))
                            }
                            (Operand::Ident(ident1), Operand::Ident(ident2)) => {
                                *unwrap_or_continue!(wires.get(&ident1))
                                    & *unwrap_or_continue!(wires.get(&ident2))
                            }
                        };

                        wires.insert(instruction.destination.clone(), value);
                    }

                    Operation::LShift(op1, op2) => {
                        let value = match (op1, op2) {
                            (Operand::Num(v1), Operand::Num(v2)) => v1 << v2,
                            (Operand::Num(value), Operand::Ident(ident)) => {
                                dbg!((&ident, value));
                                value << *unwrap_or_continue!(wires.get(&ident))
                            }
                            (Operand::Ident(ident), Operand::Num(value)) => {
                                *unwrap_or_continue!(wires.get(&ident)) << value
                            }
                            (Operand::Ident(ident1), Operand::Ident(ident2)) => {
                                *unwrap_or_continue!(wires.get(&ident1))
                                    << *unwrap_or_continue!(wires.get(&ident2))
                            }
                        };

                        wires.insert(instruction.destination.clone(), value);
                    }

                    Operation::Not(op) => {
                        let value = match op {
                            Operand::Num(value) => !value,
                            Operand::Ident(ident) => !*unwrap_or_continue!(wires.get(&ident)),
                        };

                        wires.insert(instruction.destination.clone(), value);
                    }

                    Operation::Or(op1, op2) => {
                        let value = match (op1, op2) {
                            (Operand::Num(v1), Operand::Num(v2)) => v1 | v2,
                            (Operand::Num(value), Operand::Ident(ident))
                            | (Operand::Ident(ident), Operand::Num(value)) => {
                                value | *unwrap_or_continue!(wires.get(&ident))
                            }
                            (Operand::Ident(ident1), Operand::Ident(ident2)) => {
                                *unwrap_or_continue!(wires.get(&ident1))
                                    | *unwrap_or_continue!(wires.get(&ident2))
                            }
                        };

                        wires.insert(instruction.destination.clone(), value);
                    }

                    Operation::RShift(op1, op2) => {
                        let value = match (op1, op2) {
                            (Operand::Num(v1), Operand::Num(v2)) => v1 >> v2,
                            (Operand::Num(value), Operand::Ident(ident)) => {
                                value >> *unwrap_or_continue!(wires.get(&ident))
                            }
                            (Operand::Ident(ident), Operand::Num(value)) => {
                                *unwrap_or_continue!(wires.get(&ident)) >> value
                            }
                            (Operand::Ident(ident1), Operand::Ident(ident2)) => {
                                *unwrap_or_continue!(wires.get(&ident1))
                                    >> *unwrap_or_continue!(wires.get(&ident2))
                            }
                        };

                        wires.insert(instruction.destination.clone(), value);
                    }
                }
            }
        }
    }

    let signal = wires.get("a").unwrap();

    dbg!(signal);
}
