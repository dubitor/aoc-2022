pub fn day04(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = part1(input_lines);
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(input_lines: &str) -> usize {
    input_lines
        .lines()
        .filter(|line| one_assignment_contains_other(line))
        .count()
}

fn one_assignment_contains_other(line: &str) -> bool {
    let [(low1, high1), (low2, high2)] = line
        .split(',')
        .for_each(|ass| ass.split("-").collect())
        .collect();
    low1 <= low2 && high1 >= high2 || low1 >= low2 && high1 <= high2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(
            day04(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            )
            .0,
            "2".to_string()
        )
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(day04("").1, "0".to_string())
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(day04(""), ("0".to_string(), "0".to_string()))
    }
}
