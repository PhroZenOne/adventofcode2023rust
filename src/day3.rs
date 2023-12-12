use std::fs;

pub fn parse_file_part_1(path: &str) -> u32 {
    let string = read_lines_from_file(path);
    let width = string.find("\n").expect("it should have a newline") + 1;
    let bytes = string.as_bytes();
    let mut sum = 0;
    let mut i = 0;
    let total_length = bytes.len();
    let mut checked = vec![false; total_length];
    while i < total_length {
        let center_char = bytes[i] as char;
        if center_char == '.' || center_char == '\n' || center_char == '\r' {
            checked[i] = true;
            i += 1;
            continue;
        }
        if center_char.is_numeric() {
            // not checked yet, might be read later
            i += 1;
            continue;
        }
        checked[i] = true;
        let surrounding_positions = get_targets(width, i, total_length, &checked);
        for pos in surrounding_positions {
            if checked[pos] {
                continue;
            }
            let (new_checked, value) = check_target(bytes, pos);
            sum += value;
            new_checked.iter().for_each(|&x| checked[x] = true)
        }
        i += 1;
    }
    sum
}

fn get_targets(width: usize, i: usize, total_length: usize, checked: &Vec<bool>) -> Vec<usize> {
    let c = i as i32;
    let w = width as i32;
    vec![
        c - w - 1, c - w, c - w + 1, // top row
        c - 1, c + 1, // side
        c + w - 1, c + w, c + w + 1, // bottom
    ].into_iter()
        .filter(|&x| x >= 0 && x < total_length as i32 && !checked[x as usize])
        .map(|x| x as usize)
        .collect()
}

fn check_target(bytes: &[u8], i: usize) -> (Vec<usize>, u32) {
    // if its not numeric, return only i to mark it as already checked and a total value of 0
    if !(bytes[i] as char).is_numeric() {
        return (vec![i], 0);
    }

    let mut j: i32 = 0;

    // back up to the first numeric
    while i as i32 + j > 0 && (bytes[(i as i32 + j - 1) as usize] as char).is_numeric() {
        j -= 1;
    }

    let mut checked = vec![];
    let mut s: String = String::new();
    while (bytes[(i as i32 + j) as usize] as char).is_numeric() && i as i32 + j < bytes.len() as i32 {
        s.push(bytes[(i as i32 + j) as usize] as char);
        j += 1;
        checked.push((i as i32 + j) as usize);
    }

    (checked, s.parse().expect("s should have been a number"))
}

fn read_lines_from_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("{path} file should be in place").as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse() {
        println!("day3: {}", parse_file_part_1("./data/day3/calibration.dat"));
    }
}
