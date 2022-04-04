#![warn(unused_variables, dead_code)]

use std::convert::TryFrom;
use std::convert::TryInto;

trait GetPlace {
    fn get_place(self, place: Place) -> usize;
}

#[derive(Debug)]
pub struct Operation<'a> {
    code: OpCode,
    count: u8,
    parameters: &'a [Parameter],
}

impl<'a> Operation<'_> {
    fn new(code: OpCode, count: u8, parameters: &[Parameter]) -> Operation {
        Operation {
            code,
            count,
            parameters,
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    mode: Mode,
    value: i32,
}

#[derive(PartialEq, Debug)]
pub enum OpCode {
    Add = 1,
    Mul = 2,
    Save = 3,
    Out = 4,
    Halt = 99,
}

#[derive(PartialEq, Debug)]
pub enum Mode {
    Position = 0,
    Immediate = 1,
}

impl TryFrom<usize> for OpCode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value % 100 {
            x if x == OpCode::Add as usize => Ok(OpCode::Add),
            x if x == OpCode::Mul as usize => Ok(OpCode::Mul),
            x if x == OpCode::Save as usize => Ok(OpCode::Save),
            x if x == OpCode::Out as usize => Ok(OpCode::Out),
            x if x == OpCode::Halt as usize => Ok(OpCode::Halt),
            _ => Err(()),
        }
    }
}

pub fn perform_ops(ops: &mut Vec<usize>) {
    let mut index = 0;
    let mut op: OpCode = ops[index]
        .try_into()
        .expect(format!("Failed to unwrap {:?}", ops[index]).as_str());

    while op != OpCode::Halt {
        match op {
            OpCode::Add => {
                let arg1 = ops[ops[index + 1]];
                let arg2 = ops[ops[index + 2]];
                let target_index = ops[index + 3];
                ops[target_index] = arg1 + arg2;
                index += 4;
            }
            OpCode::Mul => {
                let arg1 = ops[ops[index + 1]];
                let arg2 = ops[ops[index + 2]];
                let target_index = ops[index + 3];
                ops[target_index] = arg1 * arg2;
                index += 4;
            }
            OpCode::Save => {
                let arg = ops[index + 1];
                ops[arg] = arg;
                index += 2;
            }
            OpCode::Out => {
                let arg = ops[index + 1];
                index += 2;
            }
            _ => panic!("Invalid opcode {:?} at index {:?}", ops[index], index),
        }

        op = ops[index].try_into().unwrap();
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
        let x = 12345;
        println!("{}", (x % 100) / 10)
    }
}
