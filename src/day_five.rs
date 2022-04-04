#![warn(unused_variables, dead_code)]

trait GetPlace {
    fn get_place(self, place: Place) -> usize;
}

#[derive(PartialEq, Debug)]
pub enum OpCode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Out = 4,
    Halt = 99,
}

#[derive(PartialEq, Debug)]
pub enum Mode {
    Position = 0,
    Immediate = 1,
}

impl From<usize> for OpCode {
    fn from(value: usize) -> Self {
        match value % 100 {
            x if x == OpCode::Add as usize => OpCode::Add,
            x if x == OpCode::Mul as usize => OpCode::Mul,
            x if x == OpCode::Input as usize => OpCode::Input,
            x if x == OpCode::Out as usize => OpCode::Out,
            x if x == OpCode::Halt as usize => OpCode::Halt,
            _ => panic!("Should have received a valid OpCode, got {}", value),
        }
    }
}

impl From<usize> for Mode {
    fn from(value: usize) -> Self {
        match value {
            x if x == Mode::Immediate as usize => Mode::Immediate,
            x if x == Mode::Position as usize => Mode::Position,
            _ => panic!("Should have only received 0 or 1! Got {}", value),
        }
    }
}

fn parse_op(x: usize) -> (Mode, Mode, Mode, OpCode) {
    (
        x.get_place(Place::TenThousand).into(),
        x.get_place(Place::Thousand).into(),
        x.get_place(Place::Hundred).into(),
        OpCode::from(x),
    )
}

fn get_val(ops: &Vec<i64>, index: usize, mode: Mode) -> i64 {
    match mode {
        Mode::Position => ops[ops[index] as usize],
        Mode::Immediate => ops[index],
    }
}

pub fn perform_ops(ops: &mut Vec<i64>) {
    let mut index = 0;
    let (mut _mode3, mut mode2, mut mode1, mut op) = parse_op(ops[index] as usize);

    while op != OpCode::Halt {
        match op {
            OpCode::Add => {
                let arg1 = get_val(&ops, index + 1, mode1);
                let arg2 = get_val(&ops, index + 2, mode2);
                let target_index = ops[index + 3] as usize;
                ops[target_index] = arg1 + arg2;
                index += 4;
            }
            OpCode::Mul => {
                let arg1 = get_val(&ops, index + 1, mode1);
                let arg2 = get_val(&ops, index + 2, mode2);
                let target_index = ops[index + 3] as usize;
                ops[target_index] = arg1 * arg2;
                index += 4;
            }
            OpCode::Input => {
                let arg = ops[index + 1];
                // Hardcoded input value
                ops[arg as usize] = 1;
                index += 2;
            }
            OpCode::Out => {
                let arg = ops[ops[index + 1] as usize];
                println!("Out: {}", arg);
                index += 2;
            }
            _ => panic!("Invalid opcode {:?} at index {:?}", ops[index], index),
        }

        (_mode3, mode2, mode1, op) = parse_op(ops[index] as usize);
    }
}

pub enum Place {
    Hundred,
    Thousand,
    TenThousand,
}

impl GetPlace for usize {
    fn get_place(self, place: Place) -> usize {
        match place {
            Place::Hundred => (self % 1000) / 100,
            Place::Thousand => (self % 10000) / 1000,
            Place::TenThousand => (self % 100000) / 10000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_hundred_place() {
        let x: usize = 71201;
        assert_eq!(x.get_place(Place::Hundred), 2);
    }

    #[test]
    fn get_thousand_place() {
        let x: usize = 46201;
        assert_eq!(x.get_place(Place::Thousand), 6);
    }

    #[test]
    fn get_ten_thousand_place() {
        let x: usize = 46201;
        assert_eq!(x.get_place(Place::TenThousand), 4);
    }

    #[test]
    fn get_ten_thousand_place_when_zero() {
        let x: usize = 201;
        assert_eq!(x.get_place(Place::TenThousand), 0);
    }

    #[test]
    fn scratch() {
        let path = std::path::Path::new("resources/day_five_input.txt");
        let mut input: Vec<i64> = std::fs::read_to_string(path)
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        perform_ops(&mut input);
    }
}
