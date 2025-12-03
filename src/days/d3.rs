use crate::utils::DaySolver;

use core::cmp::Ordering;

pub struct Day3;

pub trait IterFirstMaxWithIdx: Iterator + Sized {
    #[inline]
    fn first_max_with_idx(self) -> Option<(usize, Self::Item)>
    where Self::Item: Ord,
    {
        self.first_max_by_with_idx(Ord::cmp)
    }

    #[inline]
    fn first_max_by_with_idx<F>(self, mut f: F) -> Option<(usize, Self::Item)>
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        self.enumerate().reduce(move |(ia, a), (ib, b)| {
            if f(&a, &b).is_lt() {
                (ib, b)
            } else {
                (ia, a)
            }
        })
    }
}
impl<I: Iterator> IterFirstMaxWithIdx for I { }

fn search(input: &[u8], target_len: usize) -> usize {
    let mut count = 0;
    let mut voltage = 0;

    let mut input = input;
    while count < target_len {
        let subset = &input[..(input.len()-(target_len-1-count))];
        let (next_idx, next) = subset.iter().first_max_with_idx().unwrap();
        voltage = voltage * 10 + *next as usize;
        input = &input[(next_idx + 1)..];
        count += 1;
    }
    voltage
}

impl DaySolver for Day3 {
    fn part1(&self, input: &str) -> Option<String> {
        let sum: usize = input.lines().map(|l| {
            let l: Vec<u8> = l.chars().map(|l| l.to_digit(10).unwrap() as u8).collect();
            search(&l, 2)
        }).sum();
        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let sum: usize = input.lines().map(|l| {
            let l: Vec<u8> = l.chars().map(|l| l.to_digit(10).unwrap() as u8).collect();
            search(&l, 12)
        }).sum();
        Some(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
            1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
        "};
        let solver = Day3 {};
        assert_eq!(solver.part1(input).unwrap(), "368");
        assert_eq!(solver.part2(input).unwrap(), "3233021889730");
    }
}
