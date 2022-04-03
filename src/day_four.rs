#![allow(dead_code, unused_variables)]

use std::{any, collections::HashMap};

pub fn checks(val: u64) -> bool {
    let s = val.to_string();

    let mut map: HashMap<char, u8> = HashMap::with_capacity(20);

    let char_vec: Vec<char> = s.chars().collect();

    for c in &char_vec {
        let v = map.get(c).cloned().unwrap_or_default();
        map.insert(*c, v + 1);
    }

    let mut max = 1;
    for v in map.values() {
        max = std::cmp::max(max, *v);
    }

    if max == 1 {
        return false;
    }

    for i in 0..char_vec.len() - 1 {
        let digit1 = char_vec[i].to_digit(10).unwrap();
        let digit2 = char_vec[i + 1].to_digit(10).unwrap();
        if digit1 > digit2 {
            return false;
        }
    }

    true
}

pub fn partition<T: std::cmp::PartialEq, I: IntoIterator<Item = T>>(x: I) -> Vec<Vec<T>>
where
    <I as IntoIterator>::Item: PartialEq,
{
    let mut v = vec![vec![]];
    for i in x {
        let last_v: &mut Vec<T> = v.last().unwrap().as_mut();
        let last = last_v.last();
        if last.is_none() || *last.unwrap() != i {
            v.push(vec![i]);
        } else {
            last_v.push(i);
        }
    }

    v
}

pub fn checks2(val: u64) -> bool {
    let s = val.to_string();

    let char_vec: Vec<char> = s.chars().collect();

    let mut any_adjacent = false;

    let cvl = char_vec.len();

    for i in 0..cvl - 2 {
        let c1 = char_vec[i];
        let c2 = char_vec[i + 1];
        let c3 = char_vec[i + 2];
        match (c1 == c2, c2 == c3) {
            (true, true) => {
                // breaks_rule = true;
                break;
            }
            (true, false) => {
                any_adjacent = true;
                break;
            }
            _ => continue,
        }
    }

    for i in 0..cvl - 1 {
        let digit1 = char_vec[i].to_digit(10).unwrap();
        let digit2 = char_vec[i + 1].to_digit(10).unwrap();
        if digit1 > digit2 {
            return false;
        }
    }

    true
}

pub fn solve_first(start: u64, end: u64) -> u64 {
    let mut count = 0;
    for i in start..=end {
        if checks(i) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_checks() {
        assert!(checks(111111));
        assert!(!checks(223450));
        assert!(!checks(123789));
    }

    #[test]
    fn check_checks2() {
        assert!(checks2(112233));
        assert!(!checks2(123444));
        assert!(!checks2(111122));
    }

    #[test]
    fn solve_first_input() {
        assert_eq!(solve_first(130524, 678275), 2090)
    }

    #[test]
    fn test_partition() {
        let x = partition(vec![1, 1, 2, 3, 3, 4]);
    }

    #[test]
    fn scratch() {
        // let (s1, s2) = "130254-678275".split("-");
        // println!("{:?}", s1);
        // println!("{:?}", s2);
    }
}
