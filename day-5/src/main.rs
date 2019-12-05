use std::fmt;

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    println!("Intcode input: {}", INPUT_STR);
    run_program(INPUT_STR);
}

pub fn run_program(input: &str) {
    let program_data: Vec<usize> = input
        .split(',')
        .map(|token| token.parse::<usize>().expect("Could not parse input token"))
        .collect();

    let program = IntcodeProgram::new(program_data);
    let (result, _) = program.run(12, 2);
    println!("P1 result: {}", result);
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntcodeProgram {
    exec_ptr: usize,
    ops: Vec<usize>,
    input: Vec<usize>,
    output: Vec<usize>,
}

impl IntcodeProgram {
    pub fn new(program: Vec<usize>) -> Self {
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

    fn op_add(&mut self) {
        let ptr = self.exec_ptr;
        if let [a, b, dest] = self.ops[ptr..ptr + 3] {
            self.ops[dest] = self.ops[a] + self.ops[b];
        } else {
            unreachable!("add: Wrong number of args")
        }
        self.exec_ptr += 3;
    }

    fn op_mult(&mut self) {
        let ptr = self.exec_ptr;
        if let [a, b, dest] = self.ops[ptr..ptr + 3] {
            self.ops[dest] = self.ops[a] * self.ops[b];
        } else {
            unreachable!("add: Wrong number of args")
        }
        self.exec_ptr += 3;
    }

    fn run_instruction(&mut self) {
        let opcode = self.ops[self.exec_ptr];
        self.exec_ptr += 1;

        match opcode {
            1 => self.op_add(),
            2 => self.op_mult(),
            // 3 => {
            //     // do something with input
            //     ptr + 1
            // }
            // 4 => {
            //     // do something with output
            //     ptr + 1
            // }
            99 => {
                println!("Program finished!");
                self.exec_ptr += 1;
            }
            _ => unreachable!("Unrecognized opcode {}", opcode),
        };
    }

    pub fn run(self, x: usize, y: usize) -> (usize, Self) {
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

// P1 answer: 5482655

#[cfg(test)]
mod test {
    use super::*;

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
