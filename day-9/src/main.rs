mod intcode;

use intcode::{IntcodeProgram, IntcodeResult};

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    println!("Problem 1:\n{:?}", run_from_str(INPUT_STR, vec![1]));
    println!("Problem 1:\n{:?}", run_from_str(INPUT_STR, vec![2]));
}

fn read_program(program_str: &str) -> IntcodeProgram {
    IntcodeProgram::new(
        program_str
            .split(',')
            .map(|token| token.parse::<isize>().expect("Could not parse input token"))
            .collect(),
    )
}

pub fn run_from_str(program_str: &str, input: Vec<isize>) -> Vec<isize> {
    let mut program = read_program(program_str);
    let mut outputs = Vec::new();

    let mut last_output = program.run(input);
    while let IntcodeResult::Suspend(output_value) = last_output {
        outputs.push(output_value);
        last_output = program.run(vec![]);
    }

    outputs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_quine() {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let result = run_from_str(program, vec![]);
        assert_eq!(
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            result
        )
    }

    #[test]
    fn example_size() {
        let program = "1102,34915192,34915192,7,4,7,99,0";
        let result = run_from_str(program, vec![]);
        println!("{:?}", result);
        assert!(result[0] > 999999999999999);
    }

    #[test]
    fn example_idk() {
        let program = "104,1125899906842624,99";
        assert_eq!(vec![1125899906842624], run_from_str(program, vec![]))
    }
}
