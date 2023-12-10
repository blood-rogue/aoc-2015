use itertools::Itertools;

#[derive(Clone, Copy)]
enum Register {
    A,
    B,
}

impl Register {
    fn parse(s: &str) -> Self {
        if s == "a" {
            Self::A
        } else {
            Self::B
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, usize),
    Jio(Register, usize),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let (instruction, rest) = s.split_once(' ').unwrap();

        match instruction {
            "hlf" => Self::Hlf(Register::parse(rest)),
            "tpl" => Self::Tpl(Register::parse(rest)),
            "inc" => Self::Inc(Register::parse(rest)),
            "jmp" => Self::Jmp(rest.parse().unwrap()),
            "jie" => {
                let (r, offset) = rest.split_once(", ").unwrap();
                Self::Jie(Register::parse(r), offset.parse().unwrap())
            }
            "jio" => {
                let (r, offset) = rest.split_once(", ").unwrap();
                Self::Jio(Register::parse(r), offset.parse().unwrap())
            }
            _ => panic!(),
        }
    }
}

fn main() {
    let mut a = 1;
    let mut b = 0;

    let instructions = include_str!(r"..\..\input\day23.txt")
        .lines()
        .map(Instruction::parse)
        .collect_vec();

    let mut pointer = 0usize;

    while let Some(instruction) = instructions.get(pointer) {
        match instruction {
            Instruction::Inc(r) => {
                match r {
                    Register::A => a += 1,
                    Register::B => b += 1,
                }

                pointer += 1;
            }

            Instruction::Hlf(r) => {
                match r {
                    Register::A => a /= 2,
                    Register::B => b /= 2,
                }

                pointer += 1;
            }

            Instruction::Tpl(r) => {
                match r {
                    Register::A => a *= 3,
                    Register::B => b *= 3,
                }

                pointer += 1;
            }

            Instruction::Jmp(offset) => {
                pointer = pointer.wrapping_add_signed(*offset);
            }

            Instruction::Jie(r, offset) => match r {
                Register::A => {
                    if a % 2 == 0 {
                        pointer += offset;
                    } else {
                        pointer += 1;
                    }
                }

                Register::B => {
                    if b % 2 == 0 {
                        pointer += offset;
                    } else {
                        pointer += 1;
                    }
                }
            },

            Instruction::Jio(r, offset) => match r {
                Register::A => {
                    if a == 1 {
                        pointer += offset;
                    } else {
                        pointer += 1;
                    }
                }

                Register::B => {
                    if b == 1 {
                        pointer += offset;
                    } else {
                        pointer += 1;
                    }
                }
            },
        }
    }

    dbg!(a, b);
}
