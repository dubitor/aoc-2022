fn solve_slope(input_lines: &str, right: usize, down: usize) -> u64 {
    input_lines
        .split('\n')
        .step_by(down)
        .fold((0, 0), |(acc, position), line| {
            if line.chars().cycle().nth(position) == Some('#') {
                (acc + 1, position + right)
            } else {
                (acc, position + right)
            }
        })
        .0
}

pub fn day03(input_lines: &str) -> (String, String) {
    let answer1 = solve_slope(input_lines, 3, 1);
    let right_down = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let answer2: u64 = right_down
        .iter()
        .fold(1, |acc, rd| acc * solve_slope(input_lines, rd.0, rd.1));
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(
            day03(
                r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            )
            .0,
            "7".to_string()
        )
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(
            day03(
                "r..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            )
            .1,
            "336".to_string()
        )
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(day03(""), ("0".to_string(), "0".to_string()))
    }
}
