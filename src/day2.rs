use std::fs;

#[derive(PartialEq, Debug)]
pub struct Limit {
    pub(crate) r: u32,
    pub(crate) g: u32,
    pub(crate) b: u32,
}

pub fn parse_file(path: &str, limit: &Limit) -> u32 {
    read_lines_from_file(path)
        .lines()
        .map(|row| parse_and_validate(row, limit))
        .filter(|(b, _)| *b)
        .map(|(_, game_id)| game_id)
        .sum()
}

fn read_lines_from_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect(format!("{path} file should be in place").as_str())
}

fn parse_and_validate(row: &str, limit: &Limit) -> (bool, u32) {
    let mut game: u32 = 0;
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    let mut temp: u32 = 0;
    let mut i = "Game ".len();
    let mut found_colon = false;
    let chars = row.as_bytes(); // assuming ASCII
    while i < row.len() {
        let mut current_char = chars[i] as char;
        if !found_colon {
            if current_char.is_numeric() {
                let (steps, number) = read_complete_number(chars, i);
                i += steps;
                current_char = chars[i] as char;
                game = number;
            }
            found_colon = current_char == ':';
            i += 2;
            current_char = chars[i] as char;
        }

        if current_char == ';' {
            r = 0;
            g = 0;
            b = 0;
            i += 2;
            current_char = chars[i] as char;
            temp = 0;
        }

        if current_char.is_numeric() {
            let (steps, number) = read_complete_number(chars, i);
            i += steps + 1;
            current_char = chars[i] as char;
            temp = number;
        }

        if current_char == 'g' {
            g += temp;
            if g > limit.g {
                return (false, game);
            }
            temp = 1000;
            i += "green".len();
            continue;
        } else if current_char == 'b' {
            b += temp;
            if b > limit.b {
                return (false, game);
            }
            temp = 1000;
            i += "blue".len();
            continue;
        } else if current_char == 'r' {
            r += temp;
            if r > limit.r {
                return (false, game);
            }
            temp = 1000;
            i += "red".len();
            continue;
        }
        i += 1;
    }
    return (true, game);
}

fn read_complete_number(chars: &[u8], i: usize) -> (usize, u32) {
    let mut j: usize = 0;
    let mut s: String = String::from("");
    while (chars[i + j] as char).is_numeric() {
        s.push(chars[i + j] as char);
        j += 1;
    }
    let x: u32 = s.parse().expect("s should have been a number");
    (j, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse() {
        let (passed, game_id) = parse_and_validate(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            &Limit { r: 5, g: 4, b: 9 });
        assert_eq!(game_id, 1);
        assert_eq!(passed, true);
    }

    #[test]
    fn multi_numbers_parse() {
        let (passed, game_id) = parse_and_validate(
            "Game 132: 12 blue, 213 red; 1 red, 1 green, 1 blue; 3 green",
            &Limit { r: 5, g: 4, b: 9 });
        assert_eq!(game_id, 132);
        assert_eq!(passed, false);
    }
}

