pub(crate) mod runner {
    use std::fs;

    const ZERO: &'static str = "zero";
    const ONE: &'static str = "one";
    const TWO: &'static str = "two";
    const THREE: &'static str = "three";
    const FOUR: &'static str = "four";
    const FIVE: &'static str = "five";
    const SIX: &'static str = "six";
    const SEVEN: &'static str = "seven";
    const EIGHT: &'static str = "eight";
    const NINE: &'static str = "nine";
    const NUMBERS_TO_INDEX: [&'static str; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

    pub fn parse_file(path: &str) -> u32 {
        read_from_file(path)
            .lines()
            .map(|row| parse_numbers(row))
            .filter_map(|tuple| concat_numbers(tuple))
            .sum()
    }

    fn replace_with_real_numbers(input: &str) -> String {
        let mut str: String = input.to_string();
        for (i, el) in NUMBERS_TO_INDEX.iter().enumerate() {
            let string = el.to_string();
            str = str.replace(&string, i.to_string().as_str())
        }
        return str;
    }

    fn read_from_file(path: &str) -> String {
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
        let numbers: Vec<u32> = s.chars()
            .filter_map(|x| x.to_digit(10))
            .collect();
        (numbers.first().cloned(), numbers.last().cloned())
    }

    fn findFirstNumber(s: &str) -> Option<u32> {
        let numbers: Vec<u32> = s.chars()
            .filter_map(|x| x.to_digit(10))
            .collect();
        if let (Some(first), Some(last)) = (numbers.first(), numbers.last()) {
            return (first.to_string() + &last.to_string()).parse::<u32>().ok();
        }
        return None;
    }
}