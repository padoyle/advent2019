use std::collections::HashMap;

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    println!("Result: {}", get_closest_intersection(INPUT_STR));
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

type Grid = HashMap<Point, usize>;

fn get_wire(input: &str) -> Vec<Point> {
    input.split(',').map(get_next_point).collect()
}

fn traverse_wire(grid: &mut Grid, wire: &[Point]) {
    let mut position = (0, 0);
    for delta in wire {
        let step = (delta.0.signum(), delta.1.signum());
        let target = (position.0 + delta.0, position.1 + delta.1);

        while position != target {
            position = (position.0 + step.0, position.1 + step.1);

            let new_value = match grid.get(&position) {
                Some(&value) => value + 1,
                None => 1,
            };
            grid.insert(position, new_value);
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
                        .map(usize::to_string)
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

pub fn find_intersections(grid: Grid, min_wires: usize) -> Vec<Point> {
    grid.iter()
        .filter(|(point, wires)| *wires >= &min_wires && *point != &(0, 0))
        .map(|(point, _wires)| point.clone())
        .collect()
}

pub fn get_closest_intersection(input_str: &str) -> i32 {
    let wires = read_wires(input_str);
    let mut grid = HashMap::new();
    for wire in wires {
        traverse_wire(&mut grid, wire.as_slice());
    }
    println!("grid: {:?}", grid);
    print_grid(&grid);
    println!("value at (146, 11): {:?}", grid.get(&(146, 11)));
    let intersections = find_intersections(grid, 2);
    println!("intersections: {:?}", intersections);

    intersections
        .iter()
        .map(|point| point.0.abs() + point.1.abs())
        .min()
        .expect("No intersections found")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_example() {
        let expected = 6;
        let actual = get_closest_intersection(
            r#"R8,U5,L5,D3
U7,R6,D4,L4"#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn experiment() {
        let expected = 6;
        let actual = get_closest_intersection(
            r#"R8,U5,L5,D3
U7,R6,D14,L4"#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn ex1() {
        let expected = 159;
        let actual = get_closest_intersection(
            r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn ex2() {
        let expected = 135;
        let actual = get_closest_intersection(
            r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#,
        );
        assert_eq!(expected, actual);
    }
}
