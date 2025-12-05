use std::cmp::Ordering;

use crate::utils::DaySolver;

pub struct Day5;

impl DaySolver for Day5 {
    fn part1(&self, input: &str) -> Option<String> {
        let input = input.replace("\r", "");
        let mut input = input.split("\n\n");
        let fresh = input.next().unwrap();
        let available = input.next().unwrap();

        let mut fresh_list: Vec<(usize, usize)> = Vec::new();
        fresh.lines().for_each(|l| {
            let mut l = l.split("-");
            let start = l.next().unwrap().parse().unwrap();
            let end = l.next().unwrap().parse().unwrap();
            fresh_list.push((start, end));
        });

        let mut count = 0;
        for a in available.lines() {
            let a: usize = a.parse().unwrap();
            for (s, e) in &fresh_list {
                if a >= *s && a <= *e {
                    count += 1;
                    break;
                }
            }
        }
        Some(count.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let input = input.replace("\r", "");
        let mut input = input.split("\n\n");
        let fresh = input.next().unwrap();

        let mut fresh_list: Vec<(usize, usize)> = Vec::new();
        fresh.lines().for_each(|l| {
            let mut l = l.split("-");
            let start = l.next().unwrap().parse().unwrap();
            let end = l.next().unwrap().parse().unwrap();
            fresh_list.push((start, end));
        });

        fresh_list.sort_by(|a, b| {
            if a.0.cmp(&b.0) == Ordering::Equal {
                return a.1.cmp(&b.1);
            }
            a.0.cmp(&b.0)
        });

        let mut idx = 0;
        let mut count = 0;
        while idx < fresh_list.len() {
            let start = fresh_list[idx].0;
            let mut end = fresh_list[idx].1;
            let mut next = idx + 1;
            count += (end - start) + 1;
            while next < fresh_list.len() {
                let new_start = fresh_list[next].0;   
                let new_end = fresh_list[next].1;

                if end >= new_start {
                    if end < new_end {
                        count += new_end - end;
                        end = new_end;
                    }
                } else {
                    break;
                }

                next += 1;
            }
            idx = next;
        }
        Some(count.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};
        let solver = Day5 {};
        assert_eq!(solver.part1(input).unwrap(), "3");
        assert_eq!(solver.part2(input).unwrap(), "14");
    }
}
