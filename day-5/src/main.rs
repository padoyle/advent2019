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

    pub fn has_next_instruction(&self) -> bool {
        self.exec_ptr < self.ops.len()
    }

    pub fn run_instruction(self) -> Self {
        let mut ptr = self.exec_ptr;
        let instruction = if self.ops.len() - ptr >= 4 {
            ptr += 4;
            self.ops[(ptr - 4)..ptr].to_vec()
        } else {
            ptr += 1;
            vec![99, 0, 0, 0]
        };

        let mut ops = self.ops;
        // Apply the op codes to the result, which we keep mutable
        match instruction.as_slice() {
            [1, a, b, dest] => {
                ops[*dest] = ops[*a] + ops[*b];
            }
            [2, a, b, dest] => {
                ops[*dest] = ops[*a] * ops[*b];
            }
            [99, _, _, _] => {
                // println!("Program finished");
            }
            _ => panic!("Unexpected op code {}", ops[ptr]),
        };

        Self {
            exec_ptr: ptr,
            ops,
            input: self.input,
            output: self.output,
        }
    }

    pub fn run(self, x: usize, y: usize) -> (usize, Self) {
        let mut result = self;
        result.ops[1] = x;
        result.ops[2] = y;

        while result.has_next_instruction() {
            result = result.run_instruction();
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
