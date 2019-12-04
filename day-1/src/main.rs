use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    /// The path to the input file
    path: PathBuf,
}

fn main() {
    let args = Args::from_args();

    let file = File::open(&args.path).expect("Could not find file.");
    let reader = BufReader::new(file);

    let p1_total = p1::accumulate_fuel(reader);
    println!("Problem 1 Total required: {}", p1_total);

    let file = File::open(args.path).expect("Could not find file.");
    let reader = BufReader::new(file);

    let p2_total = p2::accumulate_fuel(reader);
    println!("Problem 2 Total required: {}", p2_total);
}

// Problem 1
mod p1 {
    use super::*;

    pub fn accumulate_fuel(reader: BufReader<File>) -> i64 {
        reader
            .lines()
            .map(|line| {
                let input: i64 = line
                    .expect("Couldn't read line")
                    .parse()
                    .expect("Input was not a number");

                get_required_fuel(input)
            })
            .sum()
    }

    fn get_required_fuel(mass: i64) -> i64 {
        (mass / 3) - 2
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn check_examples() {
            assert_eq!(2, get_required_fuel(12));
            assert_eq!(2, get_required_fuel(14));
            assert_eq!(654, get_required_fuel(1969));
            assert_eq!(33583, get_required_fuel(100756));
        }
    }
}

// Problem 2
mod p2 {
    use super::*;

    pub fn accumulate_fuel(reader: BufReader<File>) -> i64 {
        reader
            .lines()
            .map(|line| {
                let input: i64 = line
                    .expect("Couldn't read line")
                    .parse()
                    .expect("Input was not a number");

                account_for_fuel(get_required_fuel(input))
            })
            .sum()
    }

    fn get_required_fuel(mass: i64) -> i64 {
        (mass / 3) - 2
    }

    fn account_for_fuel(initial_fuel: i64) -> i64 {
        let mut total = initial_fuel;
        println!("Total before accounting for fuel: {}", total);

        let mut remaining_fuel = get_required_fuel(total);
        while remaining_fuel > 0 {
            let new_remaning = get_required_fuel(remaining_fuel);
            println!("Increase total by {}", remaining_fuel);
            total += remaining_fuel;
            println!("New total: {}", total);
            remaining_fuel = new_remaning;
        }

        total
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn check_examples() {
            assert_eq!(2, account_for_fuel(2));
            assert_eq!(966, account_for_fuel(654));
            assert_eq!(50346, account_for_fuel(33583));
        }
    }
}
