use crate::utils::DaySolver;

pub struct Day2;

fn parse(i: &str) -> (usize, usize) {
    let mut r = i.split("-");
    let start: usize = r.next().unwrap().parse().unwrap();
    let end: usize = r.next().unwrap().parse().unwrap();
    (start, end)
}

fn find_invalid_in_range(start: usize, end: usize, two_only: bool) -> usize {
    let mut sum = 0;
    for c in start..=end {
        let d = c.to_string();
        let mut invalid = false;
        let min = if two_only {d.len().div_ceil(2)} else {1};
        for i in min..=d.len()/2 {
            if d.len() % i != 0 {
                continue;
            }
            let mut prev = &d[..i];
            invalid = true;
            let max = if two_only {2} else {d.len()/i};
            for rep in 2..=max {
                let next = &d[i*(rep-1)..(i*rep)];
                if prev != next {
                    invalid = false;
                    break;
                }
                prev = next;
            }
            if invalid {
                break;
            }
        }
        if invalid {
            sum += c;
        }
    }
    sum
}

impl DaySolver for Day2 {
    fn part1(&self, input: &str) -> Option<String> {
        let sum: usize = input.lines().next().unwrap().split(",").map(|i| {
            let (start, end) = parse(i);
            find_invalid_in_range(start, end, true)
        }).sum();
        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let sum: usize = input.lines().next().unwrap().split(",").map(|i| {
            let (start, end) = parse(i);
            find_invalid_in_range(start, end, false)
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
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
        "};
        let solver = Day2 {};
        assert_eq!(solver.part1(input).unwrap(), "1227775554");
        assert_eq!(solver.part2(input).unwrap(), "4174379265");
    }
}
