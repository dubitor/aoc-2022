pub fn day05(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = part1(input_lines);
    let answer2 = part2(input_lines, answer1);
    (format!("{}", answer1), format!("{}", answer2))
}

fn part2(input_lines: &str, highest_id: u16) -> u16 {
    let mut has_pass = vec![false; (highest_id + 1).into()];
    input_lines
        .split('\n')
        .map(get_seat_id)
        .for_each(|id| has_pass[id as usize] = true);
    for id in 1..(highest_id - 2) {
        let index = id as usize;
        if !has_pass[index] && has_pass[index - 1] && has_pass[index + 1] {
            return id;
        }
    }
    panic!("Couldn't find your seat :(");
}

fn part1(input_lines: &str) -> u16 {
    input_lines.split('\n').map(get_seat_id).max().unwrap()
}

fn get_seat_id(seat: &str) -> u16 {
    binary_from_seat_slice(&seat[..7]) as u16 * 8 + binary_from_seat_slice(&seat[7..]) as u16
}
// convert e.g. FBFBBFF to 0101100 (= 44)
// or RLR to 101 (= 5)
fn binary_from_seat_slice(seat_slice: &str) -> u8 {
    let digits: Vec<u8> = seat_slice.chars().map(seat_char_to_digit).collect();
    let shift = digits.len() - 1;
    digits
        .iter()
        .enumerate()
        .fold(0, |acc, (i, elem)| acc | (*elem << (shift - i)))
}

fn seat_char_to_digit(letter: char) -> u8 {
    match letter {
        'B' => 1,
        'F' => 0,
        'R' => 1,
        'L' => 0,
        _ => panic!("letter must be B, F, R, or L"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(
            day05(
                r"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
            )
            .0,
            "820".to_string()
        )
    }

    #[test]
    fn check_binary_from_seat_slice() {
        assert_eq!(binary_from_seat_slice("FBFBBFF"), 44);
        assert_eq!(binary_from_seat_slice("RLR"), 5);
    }

    #[test]
    fn check_seat_id() {
        let seat = "FBFBBFFRLR";
        assert_eq!(get_seat_id(seat), 357);
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(day05("").1, "0".to_string())
    }
}
