use std::collections::HashMap;
use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Card {
    id: usize,
    rows: usize,
}


pub fn parse_file(path: &str, _part_two: bool) -> u32 {
    let string = read_lines_from_file(path);
    let x1: Vec<Card> = string.lines().map(|x| map_to_card(x)).collect();
    let mut card_times = HashMap::new();
    x1.iter().for_each(|x| { card_times.insert(x.id, 1); });
    x1.iter().for_each(|card| {
        let multiplier = card_times.get(&card.id).expect("");
        for _i in 0..*multiplier {
            for j in card.id + 1..card.rows + card.id + 1 {
                card_times.entry(j).and_modify(|e| { *e += 1 });
            }
        }
    });
    return card_times.values().sum();
}

fn map_to_card(line: &str) -> Card {
    let regex = Regex::new(r"(\d+)").unwrap();
    let values: Vec<_> = regex
        .captures_iter(&*line)
        .filter_map(|cap| cap.get(0).map(|m| m.as_str().parse::<u8>().unwrap()))
        .collect();
    let colon_pos = line.find(":").expect("its there");
    let line_pos = line.find("|").expect("its still there");
    let winnings_pos = (line_pos - (colon_pos - 2)) / 3;
    let winning_numbers = values[1..winnings_pos].to_vec();
    let wins = values[winnings_pos..].to_vec().iter().filter(|o| winning_numbers.contains(o)).count();

    return Card {
        id: values[0] as usize,
        rows: wins,
    };
}

fn read_lines_from_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("{path} file should be in place").as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse() {
        println!("day3: {}", parse_file("./data/day4/calibration_day4.dat", false));
    }
}
