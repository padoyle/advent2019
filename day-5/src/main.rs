use std::fmt;

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    println!("Intcode input: {}", INPUT_STR);
    run_program(INPUT_STR);
}

pub fn run_program(input: &str) {
    let program_data: Vec<isize> = input
        .split(',')
        .map(|token| token.parse::<isize>().expect("Could not parse input token"))
        .collect();

    let program = IntcodeProgram::new(program_data);
    let (result, _) = program.run(12, 2);
    println!("P1 result: {}", result);
}

pub fn parse_op(opcode: isize) -> (usize, Vec<ArgMode>) {
    let op = (opcode % 100) as usize;
    let num_args = match op {
        1 | 2 => 3,
        3 | 4 => 1,
        99 => 0,
        _ => unreachable!("Unknown opcode {}", op),
    };
    let mut remaining = opcode / 100;

    let mut arg_modes = vec![0; num_args];
    for i in 0..num_args {
        arg_modes[i] = (remaining % 10) as ArgMode;
        remaining /= 10;
    }

    (op, arg_modes)
}

type ArgMode = u8;

const MODE_POS: ArgMode = 0;
const MODE_IMM: ArgMode = 1;

#[derive(Debug, PartialEq, Eq)]
pub struct IntcodeProgram {
    exec_ptr: usize,
    ops: Vec<isize>,
    input: Vec<isize>,
    output: Vec<isize>,
}

impl IntcodeProgram {
    pub fn new(program: Vec<isize>) -> Self {
        Self {
            exec_ptr: 0,
            ops: program,
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    fn has_next_instruction(&self) -> bool {
        self.exec_ptr < self.ops.len()
    }

    fn get_arg(&self, ptr: usize, mode: ArgMode) -> isize {
        if mode == MODE_POS {
            let arg_value = self.ops[ptr] as usize;
            self.ops[arg_value]
        } else if mode == MODE_IMM {
            self.ops[ptr]
        } else {
            unreachable!("invalid arg mode {}", mode);
        }
    }

    fn op_add(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let a = self.get_arg(ptr, arg_modes[0]);
        let b = self.get_arg(ptr + 1, arg_modes[1]);

        // arg 3 is always positional, and works a bit differently since
        // we store the value instead of reading it
        let dest = self.ops[ptr + 2] as usize;
        self.ops[dest] = a + b;
        self.exec_ptr += 3;
    }

    fn op_mult(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let a = self.get_arg(ptr, arg_modes[0]);
        let b = self.get_arg(ptr + 1, arg_modes[1]);

        // arg 3 is always positional, and works a bit differently since
        // we store the value instead of reading it
        let dest = self.ops[ptr + 2] as usize;
        self.ops[dest] = a * b;
        self.exec_ptr += 3;
    }

    fn run_instruction(&mut self) {
        let (opcode, arg_modes) = parse_op(self.ops[self.exec_ptr]);
        self.exec_ptr += 1;

        match opcode {
            1 => self.op_add(arg_modes),
            2 => self.op_mult(arg_modes),
            3 => unimplemented!("input"),
            4 => unimplemented!("output"),
            99 => {
                println!("Program finished!");
                self.exec_ptr += 1;
            }
            _ => unreachable!("Unrecognized opcode {}", opcode),
        };
    }

    pub fn run(self, x: isize, y: isize) -> (isize, Self) {
        let mut result = self;
        result.ops[1] = x;
        result.ops[2] = y;

        while result.has_next_instruction() {
            result.run_instruction();
        }

        (result.ops[0], result)
    }
}

impl fmt::Display for IntcodeProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string: Vec<String> = self.ops.iter().map(|num| num.to_string()).collect();

        write!(f, "{}", as_string.join(","))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn opcodes() {
        assert_eq!((2, vec![1, 1, 1]), parse_op(11102));
        assert_eq!((1, vec![0, 1, 0]), parse_op(1001));
        assert_eq!((2, vec![1, 0, 0]), parse_op(102));
        assert_eq!((4, vec![1]), parse_op(104));
        assert_eq!((3, vec![0]), parse_op(3));
    }

    #[test]
    fn examples() {
        let expected = IntcodeProgram::new(vec![2, 0, 0, 0, 99]);
        let (_, actual) = IntcodeProgram::new(vec![1, 0, 0, 0, 99]).run(0, 0);
        assert_eq!(expected.ops, actual.ops);

        let expected = IntcodeProgram::new(vec![2, 3, 0, 6, 99]);
        let (_, actual) = IntcodeProgram::new(vec![2, 3, 0, 3, 99]).run(3, 0);
        assert_eq!(expected.ops, actual.ops);

        let expected = IntcodeProgram::new(vec![2, 4, 4, 5, 99, 9801]);
        let (_, actual) = IntcodeProgram::new(vec![2, 4, 4, 5, 99, 0]).run(4, 4);
        assert_eq!(expected.ops, actual.ops);

        let expected = IntcodeProgram::new(vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
        let (_, actual) = IntcodeProgram::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).run(1, 1);
        assert_eq!(expected.ops, actual.ops);
    }
}
