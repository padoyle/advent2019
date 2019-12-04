use std::{fmt, fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    /// The path to the input file
    path: PathBuf,
}

fn main() {
    let args = Args::from_args();
    let file_string = fs::read_to_string(&args.path).expect("Could not find file.");

    println!("Intcode input: {}", file_string);
    let inputs: Vec<usize> = file_string
        .split(',')
        .map(|token| token.parse::<usize>().expect("Could not parse input token"))
        .collect();

    // P1
    let program = IntcodeProgram::new(inputs.clone());
    let (result, _) = program.run(12, 2);

    println!("P1 result: {}", result);

    // P2
    let desired_result = 19690720;
    for x in 0..100 {
        for y in 0..100 {
            let (result, _) = IntcodeProgram::new(inputs.clone()).run(x, y);
            if result == desired_result {
                println!("P2 result: {}, {}", x, y);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntcodeProgram {
    data: Vec<usize>,
}

impl IntcodeProgram {
    pub fn new(input: Vec<usize>) -> Self {
        Self { data: input }
    }

    pub fn has_instruction_at(&self, index: usize) -> bool {
        index * 4 < self.data.len()
    }

    pub fn run_instruction(self, index: usize) -> Self {
        let ptr = index * 4;
        let instruction = if self.data.len() - ptr >= 4 {
            self.data[ptr..ptr + 4].to_vec()
        } else {
            vec![99, 0, 0, 0]
        };

        let mut data = self.data;
        // Apply the op codes to the result, which we keep mutable
        match instruction.as_slice() {
            [1, a, b, dest] => {
                data[*dest] = data[*a] + data[*b];
            }
            [2, a, b, dest] => {
                data[*dest] = data[*a] * data[*b];
            }
            [99, _, _, _] => {
                // println!("Program finished");
            }
            _ => panic!("Unexpected op code {}", data[ptr]),
        };

        Self::new(data)
    }

    pub fn run(self, x: usize, y: usize) -> (usize, Self) {
        let mut result = self;
        result.data[1] = x;
        result.data[2] = y;

        let mut index = 0;
        while result.has_instruction_at(index) {
            result = result.run_instruction(index);
            index += 1;
        }

        (result.data[0], result)
    }
}

impl fmt::Display for IntcodeProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string: Vec<String> = self.data.iter().map(|num| num.to_string()).collect();

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
        assert_eq!(expected, actual);

        let expected = IntcodeProgram::new(vec![2, 3, 0, 6, 99]);
        let (_, actual) = IntcodeProgram::new(vec![2, 3, 0, 3, 99]).run(3, 0);
        assert_eq!(expected, actual);

        let expected = IntcodeProgram::new(vec![2, 4, 4, 5, 99, 9801]);
        let (_, actual) = IntcodeProgram::new(vec![2, 4, 4, 5, 99, 0]).run(4, 4);
        assert_eq!(expected, actual);

        let expected = IntcodeProgram::new(vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
        let (_, actual) = IntcodeProgram::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).run(1, 1);
        assert_eq!(expected, actual);
    }
}
