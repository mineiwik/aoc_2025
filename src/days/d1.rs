use crate::utils::DaySolver;

pub struct Day1;

impl DaySolver for Day1 {
    fn part1(&self, input: &str) -> Option<String> {
        let mut current = 50;
        let mut password = 0;
        input.lines().for_each(|i| {
            let dir = i.chars().next().unwrap();
            let mut amount: isize = i[1..].parse().unwrap();

            amount %= 100;

            current = match dir {
                'L' => current + (100 - amount),
                _ => current + amount,
            } % 100;

            if current == 0 {
                password += 1;
            }
        });
        Some(password.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mut current = 50;
        let mut password = 0;
        input.lines().for_each(|i| {
            let dir = i.chars().next().unwrap();
            let mut amount: isize = i[1..].parse().unwrap();

            password += amount/100;
            amount %= 100;

            current = match dir {
                'L' => {
                    if current > 0 && amount > current{
                        password += 1;
                    }
                    current + (100 - amount)
                }
                _ => {
                    if current < 100 && amount > (100 - current) {
                        password += 1;
                    }
                    current + amount
                }
            } % 100;

            if current == 0 {
                password += 1;
            }
        });
        Some(password.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};
        let solver = Day1 {};
        assert_eq!(solver.part1(input).unwrap(), "3");
        assert_eq!(solver.part2(input).unwrap(), "6");
    }
}
