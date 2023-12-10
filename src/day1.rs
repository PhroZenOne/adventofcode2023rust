use std::fs;
use crate::day_1_statics::NUMBER_FINDER;

pub fn parse_file(path: &str) -> u32 {
    read_lines_from_file(path)
        .lines()
        .map(|row| parse_numbers(row))
        .filter_map(|tuple| concat_numbers(tuple))
        .sum()
}

fn read_lines_from_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect(format!("{path} file should be in place").as_str())
}

fn concat_numbers((first, last): (Option<u32>, Option<u32>)) -> Option<u32> {
    if let (Some(first), Some(last)) = (first, last) {
        return (first.to_string() + &last.to_string()).parse::<u32>().ok();
    }
    return None;
}

fn parse_numbers(s: &str) -> (Option<u32>, Option<u32>) {
    let first = parse_number(s);
    let last = parse_number(s.chars().rev().collect::<String>().as_str());
    (first, last)
}

fn parse_number(s: &str) -> Option<u32> {
    let x = NUMBER_FINDER.find(s);
    x.map(|x| x.pattern().as_u32() % 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_test() {
        assert_eq!(Some(1), parse_number("1", ));
        assert_eq!(Some(1), parse_number("one", ));
        assert_eq!(Some(0), parse_number("0", ));
        assert_eq!(Some(0), parse_number("zero", ));
        assert_eq!(Some(9), parse_number("nine", ));
        assert_eq!(Some(9), parse_number("9", ));
    }

    #[test]
    fn basic_string() {
        let numbers = parse_numbers("12", );
        assert_eq!((Some(1), Some(2)), numbers);
    }

    #[test]
    fn basic_with_char_string() {
        let numbers = parse_numbers("x1x2x", );
        assert_eq!((Some(1), Some(2)), numbers);
    }

    #[test]
    fn basic_day_two() {
        let numbers = parse_numbers("two1nine", );
        assert_eq!((Some(2), Some(9)), numbers);
    }
}

