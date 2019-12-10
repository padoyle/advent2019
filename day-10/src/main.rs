use std::collections::{BTreeMap, HashSet};

static INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    println!("P1 Result: {:?}", get_most_visible(INPUT_STR));
}

fn get_most_visible(input: &str) -> (Point, usize) {
    let asteroid_map = AsteroidMap::from_input_str(input);
    let num_visible = asteroid_map.get_num_visible();

    let max = num_visible.values().max().unwrap().clone();

    num_visible
        .into_iter()
        .find(|(_point, count)| count == &max)
        .unwrap()
}

fn get_n_lasered(input: &str, n: usize) -> isize {
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

    fn get_num_visible(&self) -> BTreeMap<Point, usize> {
        let mut num_visible = BTreeMap::new();

        for asteroid in &self.asteroids {
            let mut angles = HashSet::new();
            for other in &self.asteroids {
                if asteroid == other {
                    continue;
                }

                let diff_x = other.0 - asteroid.0;
                let diff_y = other.1 - asteroid.1;

                let quad = if diff_x > 0 && diff_y > 0 {
                    1
                } else if diff_x <= 0 && diff_y > 0 {
                    2
                } else if diff_x > 0 && diff_y <= 0 {
                    3
                } else {
                    4
                };

                let diff_y = (diff_y as f64).abs();
                let diff_x = (diff_x as f64).abs();

                // Hacks for float precision lol
                let theta = (std::f64::consts::PI - (diff_y / diff_x).atan()) * FP_CORRECTION;

                angles.insert((quad, theta as isize));
            }

            num_visible.insert(asteroid.clone(), angles.len());
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
        assert_eq!(get_most_visible(input).1, 8);
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
        assert_eq!(get_most_visible(input).1, 33);
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
        assert_eq!(get_most_visible(input).1, 35);
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
        assert_eq!(get_most_visible(input).1, 41);
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
        assert_eq!(get_most_visible(input).1, 210);
    }
}
