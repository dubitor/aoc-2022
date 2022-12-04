use std::str;

pub fn day03(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = part1(input_lines);
    let answer2 = part2(input_lines);
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(input_lines: &str) -> u32 {
    input_lines
        .lines()
        .map(item_in_both_comparts)
        .map(priority)
        .sum()
}

fn item_in_both_comparts(line: &str) -> u8 {
    let len = line.len();
    let first_half = &line[..len / 2];
    let second_half = &line[len / 2..];
    for c in first_half.as_bytes() {
        if second_half.as_bytes().contains(c) {
            return *c;
        }
    }
    panic!("Couldn't find character in both compartments");
}

fn part2(input_lines: &str) -> u32 {
    let lines: Vec<&str> = input_lines.lines().collect();
    lines
        .chunks(3)
        .map(item_in_all_three_lines)
        .map(priority)
        .sum()
}

fn item_in_all_three_lines(lines: &[&str]) -> u8 {
    for c in lines[0].as_bytes() {
        if lines[1].as_bytes().contains(c) && lines[2].as_bytes().contains(c) {
            return *c;
        }
    }
    panic!("couldn't find char in all three lines");
}

fn priority(c: u8) -> u32 {
    let score = match c {
        c if c.is_ascii_lowercase() => c - 96,
        c if c.is_ascii_uppercase() => c - 38,
        _ => panic!("All characters must be ascii"),
    };
    score.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(
            day03(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )
            .0,
            "157".to_string()
        )
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(
            day03(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )
            .1,
            "70".to_string()
        )
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(day03(""), ("0".to_string(), "0".to_string()))
    }
}
