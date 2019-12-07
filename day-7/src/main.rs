mod intcode;

use intcode::{IntcodeProgram, IntcodeResult};

static INPUT_STR: &str = include_str!("../input.txt");

const STARTING_INPUT: isize = 0;

fn main() {
    let program = read_program(INPUT_STR);
    println!("Problem 1:");
    println!(
        "Best: {}",
        test_all_permutations(&program, vec![0, 1, 2, 3, 4], run_amp_sequence)
    );
    println!("Problem 2:");
    println!(
        "Best: {}",
        test_all_permutations(&program, vec![5, 6, 7, 8, 9], run_feedback_loop)
    );
}

fn get_all_permutations(input: Vec<isize>) -> Vec<Vec<isize>> {
    if input.len() == 1 {
        vec![input]
    } else {
        let mut input = input;
        let mut permutations: Vec<Vec<isize>> = Vec::new();
        let pivot = input.pop().unwrap();
        let rest_len = input.len();
        let rest = input;
        for perm in get_all_permutations(rest).drain(..) {
            for index in 0..rest_len + 1 {
                let mut perm = perm.clone();
                perm.insert(index, pivot);
                permutations.push(perm);
            }
        }

        permutations
    }
}

fn test_all_permutations<F>(program: &IntcodeProgram, phases: Vec<isize>, runner: F) -> isize
where
    F: Fn(&IntcodeProgram, Vec<isize>) -> isize,
{
    let mut best_output = 0;

    for permutation in get_all_permutations(phases).drain(..) {
        let new_output = runner(program, permutation);
        best_output = std::cmp::max(best_output, new_output);
    }

    best_output
}

fn read_program(program_str: &str) -> IntcodeProgram {
    IntcodeProgram::new(
        program_str
            .split(',')
            .map(|token| token.parse::<isize>().expect("Could not parse input token"))
            .collect(),
    )
}

pub fn run_amp_sequence(program: &IntcodeProgram, phase_settings: Vec<isize>) -> isize {
    let mut piped_value = STARTING_INPUT;
    for phase in phase_settings.into_iter() {
        // IntcodeProgram treats inputs like a stack, so the phase goes at the end in
        // order to be processed first
        let inputs = vec![piped_value, phase];
        if let IntcodeResult::Suspend(output) = program.clone().run(inputs) {
            piped_value = output;
        } else {
            unreachable!("Program halted before outputting");
        }
    }

    // This is the last output value we got
    piped_value
}

pub fn run_feedback_loop(program: &IntcodeProgram, phase_settings: Vec<isize>) -> isize {
    let amp_count = phase_settings.len();

    let mut amps = Vec::new();
    for _ in 0..amp_count {
        amps.push(program.clone());
    }

    let mut piped_value = STARTING_INPUT;
    let mut phases_initialized = false;

    loop {
        for idx in 0..phase_settings.len() {
            let inputs = if !phases_initialized {
                // IntcodeProgram treats inputs like a stack, so the phase goes at the end in
                // order to be processed first
                vec![piped_value, phase_settings[idx]]
            } else {
                // After the first loop, the phases should not be provided
                vec![piped_value]
            };
            let program_result = amps.get_mut(idx).unwrap().run(inputs);
            if let IntcodeResult::Suspend(output) = program_result {
                piped_value = output;
            } else {
                // This is the last output value we got
                return piped_value;
            }
        }

        phases_initialized = true;
    }
}

pub fn run_from_str(program_str: &str, input: Vec<isize>) -> IntcodeResult {
    let mut program = read_program(program_str);
    program.run(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_example1() {
        let program = read_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(run_amp_sequence(&program, vec![4, 3, 2, 1, 0]), 43210);
    }

    #[test]
    fn p1_example2() {
        let program = read_program(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        assert_eq!(run_amp_sequence(&program, vec![0, 1, 2, 3, 4]), 54321);
    }

    #[test]
    fn p1_example3() {
        let program = read_program(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        assert_eq!(run_amp_sequence(&program, vec![1, 0, 4, 3, 2]), 65210);
    }

    #[test]
    fn p2_example1() {
        let program = read_program(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(run_feedback_loop(&program, vec![9, 8, 7, 6, 5]), 139629729);
        assert_eq!(
            test_all_permutations(&program, vec![5, 6, 7, 8, 9], run_feedback_loop),
            139629729
        );
    }

    #[test]
    fn p2_example2() {
        let program = read_program(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        assert_eq!(run_feedback_loop(&program, vec![9, 7, 8, 5, 6]), 18216);
        assert_eq!(
            test_all_permutations(&program, vec![5, 6, 7, 8, 9], run_feedback_loop),
            18216
        );
    }
    #[test]
    fn old_p2_78examples() {
        let pos_equal = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(run_from_str(pos_equal, vec![3]), IntcodeResult::Suspend(0));
        assert_eq!(run_from_str(pos_equal, vec![8]), IntcodeResult::Suspend(1));
        assert_eq!(run_from_str(pos_equal, vec![17]), IntcodeResult::Suspend(0));

        let pos_less_than = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(
            run_from_str(pos_less_than, vec![3]),
            IntcodeResult::Suspend(1)
        );
        assert_eq!(
            run_from_str(pos_less_than, vec![8]),
            IntcodeResult::Suspend(0)
        );
        assert_eq!(
            run_from_str(pos_less_than, vec![17]),
            IntcodeResult::Suspend(0)
        );

        let imm_equal = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(run_from_str(imm_equal, vec![3]), IntcodeResult::Suspend(0));
        assert_eq!(run_from_str(imm_equal, vec![8]), IntcodeResult::Suspend(1));
        assert_eq!(run_from_str(imm_equal, vec![17]), IntcodeResult::Suspend(0));

        let imm_less_than = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(
            run_from_str(imm_less_than, vec![3]),
            IntcodeResult::Suspend(1)
        );
        assert_eq!(
            run_from_str(imm_less_than, vec![8]),
            IntcodeResult::Suspend(0)
        );
        assert_eq!(
            run_from_str(imm_less_than, vec![17]),
            IntcodeResult::Suspend(0)
        );
    }

    #[test]
    fn old_p2_jump_examples() {
        let pos_jump = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(run_from_str(pos_jump, vec![0]), IntcodeResult::Suspend(0));
        assert_eq!(run_from_str(pos_jump, vec![-8]), IntcodeResult::Suspend(1));
        assert_eq!(run_from_str(pos_jump, vec![17]), IntcodeResult::Suspend(1));

        let imm_jump = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(run_from_str(imm_jump, vec![0]), IntcodeResult::Suspend(0));
        assert_eq!(run_from_str(imm_jump, vec![-8]), IntcodeResult::Suspend(1));
        assert_eq!(run_from_str(imm_jump, vec![17]), IntcodeResult::Suspend(1));
    }

    #[test]
    fn old_p2_big_example() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(run_from_str(program, vec![0]), IntcodeResult::Suspend(999));
        assert_eq!(run_from_str(program, vec![8]), IntcodeResult::Suspend(1000));
        assert_eq!(
            run_from_str(program, vec![17]),
            IntcodeResult::Suspend(1001)
        );
    }
}
