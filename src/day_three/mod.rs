#![allow(dead_code, unused_variables)]

use std::collections::{HashMap, HashSet};
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    direction: Direction,
    distance: u32,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Command {
    pub fn new(val: &str) -> Command {
        Command {
            direction: match val.chars().nth(0) {
                Some('U') => Direction::Up,
                Some('R') => Direction::Right,
                Some('D') => Direction::Down,
                Some('L') => Direction::Left,
                _ => panic!("Invalid input! {:?}", val),
            },
            distance: val[1..]
                .parse()
                .expect(format!("Failed to parse direction from input string: {:?}", val).as_str()),
        }
    }

    pub fn travel(&self, start: Point, points: &mut HashSet<Point>) -> Point {
        let end = match self.direction {
            Direction::Up => Point {
                x: start.x,
                y: start.y + i64::from(self.distance),
            },
            Direction::Right => Point {
                x: start.x + i64::from(self.distance),
                y: start.y,
            },
            Direction::Down => Point {
                x: start.x,
                y: start.y - i64::from(self.distance),
            },
            Direction::Left => Point {
                x: start.x - i64::from(self.distance),
                y: start.y,
            },
        };

        let start_range = match self.direction {
            Direction::Right => start.x + 1,
            Direction::Left => start.x,
            Direction::Up => start.y + 1,
            Direction::Down => start.y,
        };

        let end_range = match self.direction {
            Direction::Right => end.x,
            Direction::Left => end.x - 1,
            Direction::Up => end.y,
            Direction::Down => end.y - 1,
        };

        let range = match self.direction {
            Direction::Right | Direction::Up => start_range..=end_range,
            Direction::Left | Direction::Down => end_range..=start_range,
        };

        for x in range {
            let start_range = match self.direction {
                Direction::Right | Direction::Left => {
                    points.insert(Point { x, y: start.y });
                }
                Direction::Up | Direction::Down => {
                    points.insert(Point { x: start.x, y: x });
                }
            };
        }

        end
    }

    pub fn travel_with_steps(
        &self,
        start: Point,
        mut steps: u32,
        points: &mut HashMap<Point, u32>,
    ) -> (Point, u32) {
        let end = match self.direction {
            Direction::Up => Point {
                x: start.x,
                y: start.y + i64::from(self.distance),
            },
            Direction::Right => Point {
                x: start.x + i64::from(self.distance),
                y: start.y,
            },
            Direction::Down => Point {
                x: start.x,
                y: start.y - i64::from(self.distance),
            },
            Direction::Left => Point {
                x: start.x - i64::from(self.distance),
                y: start.y,
            },
        };

        let (start_range, end_range) = match self.direction {
            Direction::Right => (start.x + 1, end.x),
            Direction::Left => (start.x - 1, end.x),
            Direction::Up => (start.y + 1, end.y),
            Direction::Down => (start.y - 1, end.y),
        };

        let range: Box::<dyn Iterator<Item = i64>> = match self.direction {
            Direction::Right | Direction::Up => 
                Box::new(start_range..=end_range),
            Direction::Left | Direction::Down => 
                Box::new((end_range..=start_range).rev()),
        };
        // Example:
        // Left: 4, starting at x: 1, y: 3 -> x: -3, y: 3
        // 0, -1, -2, -3

        for x in range {
            steps += 1;
            match self.direction {
                Direction::Right | Direction::Left => {
                    let point = Point { x, y: start.y };
                    if points.get(&point).is_none() {
                        points.insert(point, steps);
                    };
                }
                Direction::Up | Direction::Down => {
                    let point = Point { x: start.x, y: x };
                    if points.get(&point).is_none() {
                        points.insert(point, steps);
                    };
                }
            };
        }

        (end, steps)
    }
}

pub fn parse_input(input: &str) -> (Vec<Command>, Vec<Command>) {
    let mut lines = input.lines();

    let first_cmds: Vec<Command> = lines
        .nth(0)
        .unwrap()
        .split(",")
        .map(|s| Command::new(s))
        .collect();
    let second_cmds: Vec<Command> = lines
        .nth(0)
        .unwrap()
        .split(",")
        .map(|s| Command::new(s))
        .collect();

    (first_cmds, second_cmds)
}

pub fn solve_first(input: &str) -> i64 {
    let (first_cmds, second_cmds) = parse_input(input);

    let mut starting = Point { x: 0, y: 0 };
    let mut first_points = HashSet::new();

    for cmd in first_cmds {
        let new_starting = cmd.travel(starting, &mut first_points);
        starting = new_starting;
    }

    let mut starting = Point { x: 0, y: 0 };
    let mut second_points = HashSet::new();

    for cmd in second_cmds {
        let new_starting = cmd.travel(starting, &mut second_points);
        starting = new_starting;
    }

    let mut distances: Vec<_> = first_points
        .intersection(&second_points)
        .into_iter()
        .map(|Point { x, y }| x.abs() + y.abs())
        .collect();

    if distances.is_empty() {
        panic!("No intersections found!")
    }

    distances.sort_unstable();
    distances[0]
}

pub fn solve_second(input: &str) -> u32 {
    let (first_cmds, second_cmds) = parse_input(input);

    let mut start = Point { x: 0, y: 0 };
    let mut steps = 0;
    let mut first_points = HashMap::new();

    for cmd in first_cmds {
        let (new_start, new_steps) = cmd.travel_with_steps(start, steps, &mut first_points);
        start = new_start;
        steps = new_steps;
    }
    let mut start = Point { x: 0, y: 0 };
    let mut steps = 0;
    let mut second_points = HashMap::new();

    for cmd in second_cmds {
        let (new_start, new_steps) = cmd.travel_with_steps(start, steps, &mut second_points);
        start = new_start;
        steps = new_steps;
    }

    let mut inter_step = u32::MAX;

    for (k, first_step) in first_points.into_iter() {
        if let Some(second_step) = second_points.get(&k) {
            let combined = first_step + second_step;
            inter_step = std::cmp::min(inter_step, combined);
        }
    }

    inter_step
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TWO: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

    const SAMPLE_THREE: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    

    #[test]
    fn char_into_int() {
        let x = '1';
        assert_eq!(1, x.to_digit(10).unwrap())
    }

    #[test]
    fn command_new() {
        let s = "U15";
        let c = Command::new(s);

        assert_eq!(c.direction, Direction::Up);
        assert_eq!(c.distance, 15)
    }

    #[test]
    fn solve_first_sample_one() {
        let path = std::path::Path::new("resources/day_three_sample.txt");
        let contents = std::fs::read_to_string(path).unwrap();

        assert_eq!(solve_first(&contents), 6);
    }

    #[test]
    fn solve_first_sample_two() {
        assert_eq!(solve_first(SAMPLE_TWO), 159)
    }

    #[test]
    fn solve_first_sample_three() {
        assert_eq!(solve_first(SAMPLE_THREE), 135)
    }

    #[test]
    fn solve_first_input() {
        let path = std::path::Path::new("resources/day_three_input.txt");
        let contents = std::fs::read_to_string(path).unwrap();

        assert_eq!(solve_first(&contents), 1195);
    }

    #[test]
    fn solve_second_sample_one() {
        let path = std::path::Path::new("resources/day_three_sample.txt");
        let contents = std::fs::read_to_string(path).unwrap();

        assert_eq!(solve_second(&contents), 30);
    }

    #[test]
    fn solve_second_sample_two() {
        assert_eq!(solve_second(SAMPLE_TWO), 610)
    }

    #[test]
    fn solve_second_sample_three() {
        assert_eq!(solve_second(SAMPLE_THREE), 410)
    }

    #[test]
    fn solve_second_input() {
        let path = std::path::Path::new("resources/day_three_input.txt");
        let contents = std::fs::read_to_string(path).unwrap();
        assert_eq!(solve_second(&contents), 91518);
    }

    #[test]
    fn scratch() {
        for i in -1..=1 {
            println!("{:?}", i);
        }
    }
}
