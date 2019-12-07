mod intcode;

use intcode::IntcodeProgram;

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    println!("Problem 1:");
    let program = read_program(INPUT_STR);
    println!(
        "Best: {}",
        test_all_combinations(&program, vec![0, 1, 2, 3, 4])
    );
}

const STARTING_INPUT: isize = 0;

fn test_all_combinations(program: &IntcodeProgram, phases: Vec<isize>) -> isize {
    let mut best_output = 0;
    let mut phases = phases;

    for _pass in 0..phases.len() {
        for i in 0..phases.len() - 1 {
            phases.swap(i, i + 1);
            println!("testing combo {:?}", phases);
            let new_output = run_amp_sequence(program, phases.clone())[0];
            println!("{}", new_output);
            best_output = std::cmp::max(best_output, new_output);
        }
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

pub fn run_amp_sequence(program: &IntcodeProgram, phase_settings: Vec<isize>) -> Vec<isize> {
    let mut piped_values = vec![STARTING_INPUT];
    for phase in phase_settings.into_iter() {
        piped_values.push(phase);
        piped_values = program.clone().run(piped_values.clone());
    }

    piped_values
}

pub fn run_from_str(program_str: &str, input: Vec<isize>) -> Vec<isize> {
    let program = read_program(program_str);
    program.run(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let program = read_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(run_amp_sequence(&program, vec![4, 3, 2, 1, 0]), vec![43210]);
    }

    #[test]
    fn example2() {
        let program = read_program(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        assert_eq!(run_amp_sequence(&program, vec![0, 1, 2, 3, 4]), vec![54321]);
    }

    #[test]
    fn example3() {
        let program = read_program(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        assert_eq!(run_amp_sequence(&program, vec![1, 0, 4, 3, 2]), vec![65210]);
    }

    #[test]
    fn old_p2_78examples() {
        let pos_equal = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(run_from_str(pos_equal, vec![3]), vec![0]);
        assert_eq!(run_from_str(pos_equal, vec![8]), vec![1]);
        assert_eq!(run_from_str(pos_equal, vec![17]), vec![0]);

        let pos_less_than = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(run_from_str(pos_less_than, vec![3]), vec![1]);
        assert_eq!(run_from_str(pos_less_than, vec![8]), vec![0]);
        assert_eq!(run_from_str(pos_less_than, vec![17]), vec![0]);

        let imm_equal = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(run_from_str(imm_equal, vec![3]), vec![0]);
        assert_eq!(run_from_str(imm_equal, vec![8]), vec![1]);
        assert_eq!(run_from_str(imm_equal, vec![17]), vec![0]);

        let imm_less_than = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(run_from_str(imm_less_than, vec![3]), vec![1]);
        assert_eq!(run_from_str(imm_less_than, vec![8]), vec![0]);
        assert_eq!(run_from_str(imm_less_than, vec![17]), vec![0]);
    }

    #[test]
    fn old_p2_jump_examples() {
        let pos_jump = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(run_from_str(pos_jump, vec![0]), vec![0]);
        assert_eq!(run_from_str(pos_jump, vec![-8]), vec![1]);
        assert_eq!(run_from_str(pos_jump, vec![17]), vec![1]);

        let imm_jump = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(run_from_str(imm_jump, vec![0]), vec![0]);
        assert_eq!(run_from_str(imm_jump, vec![-8]), vec![1]);
        assert_eq!(run_from_str(imm_jump, vec![17]), vec![1]);
    }

    #[test]
    fn old_p2_big_example() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(run_from_str(program, vec![0]), vec![999]);
        assert_eq!(run_from_str(program, vec![8]), vec![1000]);
        assert_eq!(run_from_str(program, vec![17]), vec![1001]);
    }
}
