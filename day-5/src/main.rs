use std::fmt;

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    println!("Intcode input: {}", INPUT_STR);
    run_program(INPUT_STR, 1);
}

pub fn run_program(program_str: &str, input: isize) {
    let program_data: Vec<isize> = program_str
        .split(',')
        .map(|token| token.parse::<isize>().expect("Could not parse input token"))
        .collect();

    let program = IntcodeProgram::new(program_data, input);
    let output = program.run();
    println!("P1 result: {:?}", output);
}

pub fn parse_op(opcode: isize) -> (usize, Vec<ArgMode>) {
    let op = (opcode % 100) as usize;
    let num_args = match op {
        1 | 2 => 3,
        3 | 4 => 1,
        99 => 0,
        _ => unreachable!("Unknown opcode {}", opcode),
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
    ops: Vec<isize>,
    exec_ptr: usize,

    input: isize,
    output: Vec<isize>,
}

impl IntcodeProgram {
    pub fn new(ops: Vec<isize>, input: isize) -> Self {
        Self {
            ops,
            exec_ptr: 0,

            input,
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

    fn op_input(&mut self, _arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let dest = self.ops[ptr] as usize;

        self.ops[dest] = self.input;

        self.exec_ptr += 1;
    }

    fn op_output(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let output_value = self.get_arg(ptr, arg_modes[0]);

        self.output.push(output_value);
        self.exec_ptr += 1;
    }

    fn run_instruction(&mut self) {
        println!("Run instruction @ {}", self.exec_ptr + 1);
        let (opcode, arg_modes) = parse_op(self.ops[self.exec_ptr]);
        self.exec_ptr += 1;

        match opcode {
            1 => self.op_add(arg_modes),
            2 => self.op_mult(arg_modes),
            3 => self.op_input(arg_modes),
            4 => self.op_output(arg_modes),
            99 => {
                println!("Program finished!");
                self.exec_ptr = self.ops.len();
            }
            _ => unreachable!("Unrecognized opcode {}", opcode),
        };
    }

    pub fn run(self) -> Vec<isize> {
        let mut result = self;
        while result.has_next_instruction() {
            result.run_instruction();
        }

        result.output
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
}
