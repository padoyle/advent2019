use std::fmt;

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    println!("Intcode input: {}", INPUT_STR);

    println!("Part 1:");
    let output = run_program(INPUT_STR, 1);
    println!("{:?}", output);

    println!("Part 2:");
    let output = run_program(INPUT_STR, 5);
    println!("{:?}", output);
}

pub fn run_program(program_str: &str, input: isize) -> Vec<isize> {
    let program_data: Vec<isize> = program_str
        .split(',')
        .map(|token| token.parse::<isize>().expect("Could not parse input token"))
        .collect();

    let program = IntcodeProgram::new(program_data, input);
    program.run()
}

pub fn parse_op(opcode: isize) -> (usize, Vec<ArgMode>) {
    let op = (opcode % 100) as usize;
    let num_args = match op {
        1 | 2 => 3,
        3 | 4 => 1,
        5 | 6 => 2,
        7 | 8 => 3,
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

    fn op_jump_if_true(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let condition = self.get_arg(ptr, arg_modes[0]) != 0;

        if condition {
            // Jump to the designated location
            self.exec_ptr = self.get_arg(ptr + 1, arg_modes[1]) as usize;
        } else {
            // Move on to the next op
            self.exec_ptr += 2;
        }
    }

    fn op_jump_if_false(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let condition = self.get_arg(ptr, arg_modes[0]) == 0;

        if condition {
            // Jump to the designated location
            self.exec_ptr = self.get_arg(ptr + 1, arg_modes[1]) as usize;
        } else {
            // Move on to the next op
            self.exec_ptr += 2;
        }
    }

    fn op_less_than(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let a = self.get_arg(ptr, arg_modes[0]);
        let b = self.get_arg(ptr + 1, arg_modes[1]);

        let result = if a < b { 1 } else { 0 };
        let dest = self.ops[ptr + 2] as usize;
        self.ops[dest] = result;

        self.exec_ptr += 3;
    }

    fn op_equals(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let a = self.get_arg(ptr, arg_modes[0]);
        let b = self.get_arg(ptr + 1, arg_modes[1]);

        let result = if a == b { 1 } else { 0 };
        let dest = self.ops[ptr + 2] as usize;
        self.ops[dest] = result;

        self.exec_ptr += 3;
    }

    fn run_instruction(&mut self) {
        let (opcode, arg_modes) = parse_op(self.ops[self.exec_ptr]);
        self.exec_ptr += 1;

        match opcode {
            1 => self.op_add(arg_modes),
            2 => self.op_mult(arg_modes),
            3 => self.op_input(arg_modes),
            4 => self.op_output(arg_modes),
            5 => self.op_jump_if_true(arg_modes),
            6 => self.op_jump_if_false(arg_modes),
            7 => self.op_less_than(arg_modes),
            8 => self.op_equals(arg_modes),
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

    #[test]
    fn p2_78examples() {
        let pos_equal = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(run_program(pos_equal, 3), vec![0]);
        assert_eq!(run_program(pos_equal, 8), vec![1]);
        assert_eq!(run_program(pos_equal, 17), vec![0]);

        let pos_less_than = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(run_program(pos_less_than, 3), vec![1]);
        assert_eq!(run_program(pos_less_than, 8), vec![0]);
        assert_eq!(run_program(pos_less_than, 17), vec![0]);

        let imm_equal = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(run_program(imm_equal, 3), vec![0]);
        assert_eq!(run_program(imm_equal, 8), vec![1]);
        assert_eq!(run_program(imm_equal, 17), vec![0]);

        let imm_less_than = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(run_program(imm_less_than, 3), vec![1]);
        assert_eq!(run_program(imm_less_than, 8), vec![0]);
        assert_eq!(run_program(imm_less_than, 17), vec![0]);
    }

    #[test]
    fn p2_jump_examples() {
        let pos_jump = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(run_program(pos_jump, 0), vec![0]);
        assert_eq!(run_program(pos_jump, -8), vec![1]);
        assert_eq!(run_program(pos_jump, 17), vec![1]);

        let imm_jump = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(run_program(imm_jump, 0), vec![0]);
        assert_eq!(run_program(imm_jump, -8), vec![1]);
        assert_eq!(run_program(imm_jump, 17), vec![1]);
    }

    #[test]
    fn p2_big_example() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(run_program(program, 0), vec![999]);
        assert_eq!(run_program(program, 8), vec![1000]);
        assert_eq!(run_program(program, 17), vec![1001]);
    }
}
