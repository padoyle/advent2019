use std::fmt;

pub fn parse_op(opcode: isize) -> (usize, Vec<ArgMode>) {
    let op = (opcode % 100) as usize;
    let num_args = match op {
        1 | 2 => 3,
        3 | 4 | 9 => 1,
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
const MODE_REL: ArgMode = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntcodeResult {
    Suspend(isize),
    Halt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntcodeProgram {
    ops: Vec<isize>,
    exec_ptr: usize,
    relative_base: isize,

    input: Vec<isize>,
    output: Option<isize>,
}

impl IntcodeProgram {
    pub fn new(ops: Vec<isize>) -> Self {
        Self {
            ops,
            exec_ptr: 0,
            relative_base: 0,

            input: Vec::new(),
            output: None,
        }
    }

    fn has_next_instruction(&self) -> bool {
        self.exec_ptr < self.ops.len()
    }

    fn extend_memory(&mut self, target_location: usize) {
        let start = self.ops.len();
        for _ in start - 1..target_location + 1 {
            self.ops.push(0);
        }
    }

    fn get_value(&mut self, target_location: usize) -> isize {
        if target_location > self.ops.len() {
            self.extend_memory(target_location);
        }

        self.ops[target_location]
    }

    fn set_value(&mut self, target_location: usize, new_value: isize) {
        if target_location > self.ops.len() {
            self.extend_memory(target_location);
        }

        self.ops[target_location] = new_value;
    }

    fn get_target_address(&mut self, ptr: usize, mode: ArgMode) -> usize {
        if mode == MODE_POS {
            self.get_value(ptr) as usize
        } else if mode == MODE_IMM {
            ptr
        } else if mode == MODE_REL {
            self.get_value((ptr as isize + self.relative_base) as usize) as usize
        } else {
            unreachable!("invalid arg mode {}", mode);
        }
    }

    fn get_arg(&mut self, ptr: usize, mode: ArgMode) -> isize {
        let target_address = self.get_target_address(ptr, mode);
        self.get_value(target_address)
    }

    fn op_add(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let a = self.get_arg(ptr, arg_modes[0]);
        let b = self.get_arg(ptr + 1, arg_modes[1]);

        // arg 3 is always positional, and works a bit differently since
        // we store the value instead of reading it
        let dest = self.get_target_address(ptr + 2, arg_modes[2]);
        self.set_value(dest, a + b);
        self.exec_ptr += 3;
    }

    fn op_mult(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let a = self.get_arg(ptr, arg_modes[0]);
        let b = self.get_arg(ptr + 1, arg_modes[1]);

        // arg 3 is always positional, and works a bit differently since
        // we store the value instead of reading it
        let dest = self.get_target_address(ptr + 2, arg_modes[2]);
        self.set_value(dest, a * b);
        self.exec_ptr += 3;
    }

    fn op_input(&mut self, _arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let dest = self.get_value(ptr) as usize;

        let value = self
            .input
            .pop()
            .expect("Program required input but none was remaining");
        self.set_value(dest, value);

        self.exec_ptr += 1;
    }

    fn op_output(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let output_value = self.get_arg(ptr, arg_modes[0]);

        self.output = Some(output_value);
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
        let dest = self.get_target_address(ptr + 2, arg_modes[2]);
        self.set_value(dest, result);

        self.exec_ptr += 3;
    }

    fn op_equals(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let a = self.get_arg(ptr, arg_modes[0]);
        let b = self.get_arg(ptr + 1, arg_modes[1]);

        let result = if a == b { 1 } else { 0 };
        let dest = self.get_target_address(ptr + 2, arg_modes[2]);
        self.set_value(dest, result);

        self.exec_ptr += 3;
    }

    fn op_relataive_offset(&mut self, arg_modes: Vec<ArgMode>) {
        let ptr = self.exec_ptr;
        let arg_value = self.get_arg(ptr, arg_modes[0]);

        self.relative_base += arg_value;

        self.exec_ptr += 1;
    }

    fn run_instruction(&mut self) {
        let (opcode, arg_modes) = parse_op(self.ops[self.exec_ptr]);
        self.exec_ptr += 1;

        println!(
            "Execute op: {} {:?}; rb: {}",
            opcode, arg_modes, self.relative_base
        );

        match opcode {
            1 => self.op_add(arg_modes),
            2 => self.op_mult(arg_modes),
            3 => self.op_input(arg_modes),
            4 => self.op_output(arg_modes),
            5 => self.op_jump_if_true(arg_modes),
            6 => self.op_jump_if_false(arg_modes),
            7 => self.op_less_than(arg_modes),
            8 => self.op_equals(arg_modes),
            9 => self.op_relataive_offset(arg_modes),
            99 => {
                self.exec_ptr = self.ops.len();
            }
            _ => unreachable!("Unrecognized opcode {}", opcode),
        };
    }

    pub fn run(&mut self, input: Vec<isize>) -> IntcodeResult {
        println!("Run with input: {:?}", input);
        self.input = input;

        while self.has_next_instruction() {
            self.run_instruction();

            if let Some(output) = self.output.take() {
                println!("Output: {}", output);
                return IntcodeResult::Suspend(output);
            }
        }

        IntcodeResult::Halt
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
