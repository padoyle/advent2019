use std::collections::HashMap;

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let orbits = populate_orbits(INPUT_STR);

    println!("P1: {}", find_checksum(&orbits));
    println!("P2: {}", min_dist_to_santa(&orbits));
}

type Orbits = HashMap<&'static str, &'static str>;

const CENTER_OF_MASS: &str = "COM";
const YOU: &str = "YOU";
const SANTA: &str = "SAN";

fn populate_orbits(input: &'static str) -> Orbits {
    let mut orbits = HashMap::new();

    for line in input.lines() {
        let tokens: Vec<&'static str> = line.split(')').collect();
        if let [center, orbiter] = tokens.as_slice() {
            orbits.insert(*orbiter, *center);
        } else {
            unreachable!("Invalid input format: {}", line);
        }
    }

    orbits
}

fn find_checksum(orbits: &Orbits) -> usize {
    let mut count = 0;

    let all_orbiters: Vec<&'static str> = orbits
        .keys()
        .into_iter()
        .map(|orbitter| *orbitter)
        .collect();

    for orbitter in all_orbiters {
        let mut parent = orbitter;
        while parent != CENTER_OF_MASS {
            parent = orbits
                .get(parent)
                .expect("Couldn't find orbitter's orbittee");
            count += 1;
        }
    }

    count
}

fn path_to_com(orbits: &Orbits, start: &'static str) -> Vec<&'static str> {
    let mut path = Vec::new();

    let mut loc = start;
    while loc != CENTER_OF_MASS {
        loc = orbits.get(loc).expect("Couldn't find orbitter's orbittee");
        path.push(loc);
    }

    path
}

fn min_dist_to_santa(orbits: &Orbits) -> usize {
    let mut you_path = path_to_com(orbits, YOU);
    let mut santa_path = path_to_com(orbits, SANTA);

    while you_path.last() == santa_path.last() {
        you_path.pop();
        santa_path.pop();
    }

    you_path.len() + santa_path.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_example() {
        let map = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;
        assert_eq!(42, find_checksum(&populate_orbits(map)));
    }

    #[test]
    fn p2_example() {
        let map = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;
        assert_eq!(4, min_dist_to_santa(&populate_orbits(map)));
    }
}
