use std::collections::HashMap;

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let grid = generate_grid(INPUT_STR);

    println!("P1 Result: {}", get_closest_intersection(&grid));
    println!("P2 Result: {}", get_fewest_steps(&grid));
}

type Point = (i32, i32);

pub fn get_next_point(path: &str) -> Point {
    if path.is_empty() {
        panic!("Reached invalid specifier {}", path);
    }
    let (direction, distance) = path.split_at(1);
    let distance: i32 = distance.parse().expect("Invalid input");

    match direction {
        "L" => (-distance, 0),
        "R" => (distance, 0),
        "U" => (0, distance),
        "D" => (0, -distance),
        _ => panic!("Invalid input {}{}", direction, distance),
    }
}

type WireStep = Vec<usize>;
type Grid = HashMap<Point, WireStep>;

fn get_wire(input: &str) -> Vec<Point> {
    input.split(',').map(get_next_point).collect()
}

fn traverse_wire(grid: &mut Grid, wire: &[Point], wire_index: usize) {
    let mut position = (0, 0);
    let mut step_count = 0;
    for delta in wire {
        let step = (delta.0.signum(), delta.1.signum());
        let target = (position.0 + delta.0, position.1 + delta.1);

        while position != target {
            step_count += 1;
            position = (position.0 + step.0, position.1 + step.1);

            if let Some(wires) = grid.get_mut(&position) {
                let same_wire = wires[wire_index];
                let new_value = if same_wire == 0 {
                    step_count
                } else {
                    std::cmp::min(same_wire, step_count)
                };
                wires[wire_index] = new_value;
            } else {
                let mut new_value = vec![0, 0];
                new_value[wire_index] = step_count;

                grid.insert(position, new_value);
            }
        }
    }
}

fn print_grid(grid: &Grid) {
    let max_x = grid.keys().map(|(x, _y)| x).max().unwrap().clone();
    let max_y = grid.keys().map(|(_x, y)| y).max().unwrap().clone();
    let min_x = grid.keys().map(|(x, _y)| x).min().unwrap().clone();
    let min_y = grid.keys().map(|(_x, y)| y).min().unwrap().clone();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                print!("O ");
            } else {
                print!(
                    "{} ",
                    grid.get(&(x, y))
                        .map(|wires| count_hits(wires).to_string())
                        .unwrap_or(".".to_string())
                );
            }
        }
        println!();
    }
}

fn read_wires(value: &str) -> Vec<Vec<Point>> {
    println!("lines: {}", value.lines().count());
    value.lines().map(get_wire).collect()
}

fn count_hits(wire_step: &[usize]) -> usize {
    let mut hits = 0;
    for step in wire_step {
        if step > &0 {
            hits += 1;
        }
    }

    hits
}

pub fn find_intersections(grid: &Grid, min_wires: usize) -> Vec<Point> {
    grid.into_iter()
        .filter(|(point, wires)| count_hits(wires) >= min_wires && *point != &(0, 0))
        .map(|(point, wires)| {
            println!("Found hit {:?}: {:?}", point, wires);
            point.clone()
        })
        .collect()
}

pub fn generate_grid(input_str: &str) -> Grid {
    let wires = read_wires(input_str);
    let mut grid = HashMap::new();
    for (i, wire) in wires.iter().enumerate() {
        // println!("wire {}", i);
        traverse_wire(&mut grid, wire.as_slice(), i);
    }
    // print_grid(&grid);

    grid
}

pub fn get_closest_intersection(grid: &Grid) -> i32 {
    let intersections = find_intersections(grid, 2);

    intersections
        .iter()
        .map(|point| point.0.abs() + point.1.abs())
        .min()
        .expect("No intersections found")
}

pub fn get_fewest_steps(grid: &Grid) -> usize {
    let intersections = find_intersections(grid, 2);

    intersections
        .iter()
        .map(|point| grid.get(point).unwrap().iter().sum())
        .min()
        .expect("No intersections found")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_hits_test() {
        assert_eq!(0, count_hits(&vec![0, 0]));
        assert_eq!(1, count_hits(&vec![0, 3]));
        assert_eq!(1, count_hits(&vec![1, 0]));
        assert_eq!(2, count_hits(&vec![8, 17]));
    }

    #[test]
    fn simple_example() {
        let grid = generate_grid(
            r#"R8,U5,L5,D3
U7,R6,D4,L4"#,
        );
        let expected_closest = 6;
        let actual_closest = get_closest_intersection(&grid);
        assert_eq!(expected_closest, actual_closest);

        let expexted_fewest = 30;
        let actual_fewest = get_fewest_steps(&grid);
        assert_eq!(expexted_fewest, actual_fewest);
    }

    #[test]
    fn experiment() {
        let grid = generate_grid(
            r#"R8,U5,L5,D3
U7,R6,D14,L4"#,
        );
        let expected_closest = 6;
        let actual_closest = get_closest_intersection(&grid);
        assert_eq!(expected_closest, actual_closest);
    }

    #[test]
    fn ex1() {
        let grid = generate_grid(
            r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#,
        );
        let expected_closest = 159;
        let actual_closest = get_closest_intersection(&grid);
        assert_eq!(expected_closest, actual_closest);

        let expexted_fewest = 610;
        let actual_fewest = get_fewest_steps(&grid);
        assert_eq!(expexted_fewest, actual_fewest);
    }

    #[test]
    fn ex2() {
        let grid = generate_grid(
            r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#,
        );
        let expected_closest = 135;
        let actual_closest = get_closest_intersection(&grid);
        assert_eq!(expected_closest, actual_closest);

        let expexted_fewest = 410;
        let actual_fewest = get_fewest_steps(&grid);
        assert_eq!(expexted_fewest, actual_fewest);
    }
}
