use std::fs;

fn num_to_fuel(n: i64) -> i64 {
    (n / 3) - 2
}

fn str_to_fuel(n: &str) -> i64 {
    num_to_fuel(n.parse::<i64>().unwrap())
}

pub fn solve_first(path: &std::path::Path) -> i64 {
    let contents = fs::read_to_string(path).expect(format!("Missing file {:?}", path).as_str());
    let nums: Vec<i64> = contents
        .split("\n")
        .into_iter()
        .map(|x| str_to_fuel(x))
        .collect();

    let mut acc = 0;
    for num in nums {
        acc += num;
    }
    acc
}

fn str_to_all_fuel(n: &str) -> i64 {
    let mut current_fuel = str_to_fuel(n);
    let mut total_fuel = 0;
    while current_fuel > 0 {
        total_fuel += current_fuel;
        current_fuel = num_to_fuel(current_fuel);
    }
    total_fuel
}

pub fn solve_second(path: &std::path::Path) -> i64 {
    let contents = fs::read_to_string(path).expect(format!("Missing file {:?}", path).as_str());

    contents
        .split("\n")
        .into_iter()
        .map(|x| str_to_all_fuel(x))
        .reduce(|acc, x| acc + x)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solves_first_sample() {
        let path = "files/day_one_sample.txt";
        let num = solve_first(std::path::Path::new(path));
        assert_eq!(33583, num);
    }

    #[test]
    fn solves_first_input() {
        let path = "files/day_one_input.txt";
        let num = solve_first(std::path::Path::new(path));
        assert_eq!(3434390, num);
    }

    #[test]
    fn solves_second_sample() {
        let path = "files/day_one_sample.txt";
        let num = solve_second(std::path::Path::new(path));
        assert_eq!(50346, num);
    }

    #[test]
    fn solves_second_input() {
        let path = "files/day_one_input.txt";
        let num = solve_second(std::path::Path::new(path));
        assert_eq!(5148724, num);
    }
}
