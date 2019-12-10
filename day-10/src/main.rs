use std::collections::{BTreeMap, HashSet};

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let mut asteroid_map = AsteroidMap::from_input_str(INPUT_STR);
    println!("P1 Result: {:?}", get_most_visible(&asteroid_map));

    println!("P2 Result: {}", get_n_lasered(&mut asteroid_map, 200));
}

fn get_most_visible(asteroid_map: &AsteroidMap) -> (Point, usize) {
    let num_visible = asteroid_map.get_num_visible();
    let max = num_visible.values().max().unwrap().clone();

    num_visible
        .into_iter()
        .find(|(_point, count)| count == &max)
        .unwrap()
}

fn get_n_lasered(asteroid_map: &mut AsteroidMap, n: usize) -> isize {
    let (station, _) = get_most_visible(&asteroid_map);

    let mut count = 0;
    while count < n {
        // Use BTreeMap + the fact that get_visible is carefully structured to have clockwise keys
        let asteroids = asteroid_map.get_visible(&station);
        println!("{} asteroids this round", asteroids.len());
        for (_ordering, point) in asteroids {
            asteroid_map.laser_asteroid(&point);
            count += 1;

            let vector = [point.0 - station.0, point.1 - station.1];
            println!(
                "{}: Lasering [{}, {}] {:?}",
                count, vector[0], vector[1], point
            );

            if count == n {
                return point.0 * 100 + point.1;
            }
        }

        println!("lasered {} asteroids total, finding new map", count);
    }

    0
}

const FP_CORRECTION: f64 = 1e10;

type Point = (isize, isize);

struct AsteroidMap {
    asteroids: HashSet<Point>,
}

impl AsteroidMap {
    fn from_input_str(input: &str) -> Self {
        let mut asteroids = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, presence) in line.chars().enumerate() {
                if presence == '#' {
                    asteroids.insert((x as isize, y as isize));
                }
            }
        }

        Self { asteroids }
    }

    fn get_visible(&self, asteroid: &Point) -> BTreeMap<(usize, isize), Point> {
        let mut visible = BTreeMap::new();
        for other in &self.asteroids {
            if asteroid == other {
                continue;
            }
            let diff_x = other.0 - asteroid.0;
            let diff_y = other.1 - asteroid.1;

            // Quad numbering is wonky for sequence reasons for part 2
            let quad = if diff_x >= 0 && diff_y < 0 {
                1
            } else if diff_x >= 0 && diff_y >= 0 {
                2
            } else if diff_x < 0 && diff_y >= 0 {
                3
            } else {
                4
            };

            let diff_y = (diff_y as f64).abs();
            let diff_x = (diff_x as f64).abs();

            // Hacks for float precision lol
            let theta = (diff_y / diff_x).atan();
            let theta_clockwise = if quad == 1 || quad == 3 {
                (std::f64::consts::PI / 2.0) - theta
            } else {
                theta
            };
            let theta_corrected = (theta_clockwise * FP_CORRECTION) as isize;

            visible.insert((quad, theta_corrected), other.clone());
        }

        visible
    }

    fn get_num_visible(&self) -> BTreeMap<Point, usize> {
        let mut num_visible = BTreeMap::new();

        for asteroid in &self.asteroids {
            num_visible.insert(asteroid.clone(), self.get_visible(asteroid).len());
        }

        num_visible
    }

    fn laser_asteroid(&mut self, asteroid: &Point) {
        self.asteroids.remove(asteroid);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1explanation_example() {
        let input = ".#..#
.....
#####
....#
...##";
        assert_eq!(get_most_visible(&AsteroidMap::from_input_str(input)).1, 8);
    }

    #[test]
    fn p1example1() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        assert_eq!(get_most_visible(&AsteroidMap::from_input_str(input)).1, 33);
    }

    #[test]
    fn p1example2() {
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        assert_eq!(get_most_visible(&AsteroidMap::from_input_str(input)).1, 35);
    }

    #[test]
    fn p1example3() {
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        assert_eq!(get_most_visible(&AsteroidMap::from_input_str(input)).1, 41);
    }

    #[test]
    fn p1example4() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(get_most_visible(&AsteroidMap::from_input_str(input)).1, 210);
    }

    #[test]
    fn p2example() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(
            get_n_lasered(&mut AsteroidMap::from_input_str(input), 200),
            802
        );
    }
}
