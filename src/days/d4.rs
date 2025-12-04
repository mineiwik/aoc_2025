use std::collections::HashSet;

use num::Complex;

use crate::utils::DaySolver;

pub struct Day4;

const NEIGHBORS: &[Complex<isize>] = &[
    Complex::new(1, 0),
    Complex::new(1, 1),
    Complex::new(0, 1),
    Complex::new(-1, 1),
    Complex::new(-1, 0),
    Complex::new(-1, -1),
    Complex::new(0, -1),
    Complex::new(1, -1),
    
];

impl DaySolver for Day4 {
    fn part1(&self, input: &str) -> Option<String> {
        let mut rolls = HashSet::new();
        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                if c == '@' {
                    rolls.insert(Complex::new(x as isize, y as isize));
                }
            })
        });

        let mut sum = 0;

        for r in &rolls {
            let mut count = 0;
            for n in NEIGHBORS {
                if rolls.contains(&(n+r)) {
                    count += 1;
                    if count > 3 {
                        break
                    }
                }
            }
            if count < 4 {
                sum += 1;
            }
            
        }

        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mut rolls = HashSet::new();
        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                if c == '@' {
                    rolls.insert(Complex::new(x as isize, y as isize));
                }
            })
        });

        let mut sum = 0;
        
        
        loop {
            let mut done = HashSet::new();
            let mut new = 0;
            for r in &rolls {
                let mut count = 0;
                for n in NEIGHBORS {
                    if rolls.contains(&(n+r)) {
                        count += 1;
                        if count > 3 {
                            break
                        }
                    }
                }
                if count < 4 {
                    done.insert(*r);
                    new += 1;
                }
            }

            rolls.retain(|k| !done.contains(k));

            sum += new;
            if new == 0 {
                break;
            }
        }

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
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};
        let solver = Day4 {};
        assert_eq!(solver.part1(input).unwrap(), "13");
        assert_eq!(solver.part2(input).unwrap(), "43");
    }
}
