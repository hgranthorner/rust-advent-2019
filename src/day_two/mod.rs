use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Write;

#[derive(PartialEq, Debug)]
pub enum OpCode {
    Invalid = 0,
    Add = 1,
    Mul = 2,
    Halt = 99,
}

impl TryFrom<usize> for OpCode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == OpCode::Add as usize => Ok(OpCode::Add),
            x if x == OpCode::Mul as usize => Ok(OpCode::Mul),
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
            }
            OpCode::Mul => {
                let arg1 = ops[ops[index + 1]];
                let arg2 = ops[ops[index + 2]];
                let target_index = ops[index + 3];
                ops[target_index] = arg1 * arg2;
            }
            _ => panic!("Invalid opcode {:?} at index {:?}", ops[index], index),
        }

        index += 4;
        op = ops[index].try_into().unwrap();
    }
}

pub fn solve_first(s: &str) -> usize {
    let mut ops: Vec<usize> = s
        .split(",")
        .map(|x| {
            println!("{:?}", x);
            std::io::stdout().flush();
            x.parse().unwrap()
        })
        .collect();

    perform_ops(&mut ops);

    ops[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn solves_first_sample() {
        let val = fs::read_to_string("resources/day_two_sample.txt").unwrap();
        let num = solve_first(&val);
        assert_eq!(3500, num);
    }

    #[test]
    fn solves_first_input() {
        let path = "resources/day_two_input.txt";
        let num = solve_first(path);
        assert_eq!(3931283, num);
    }

    // #[test]
    // fn solves_second_sample() {
    //     let path = "resources/day_two_sample.txt";
    //     let num = solve_second(path);
    //     assert_eq!(50346, num);
    // }

    // #[test]
    // fn solves_second_input() {
    //     let path = "resources/day_two_input.txt";
    //     let num = solve_second(path);
    //     assert_eq!(5148724, num);
    // }
}
