use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn get_calibration_value_v1(s: &str) -> u32 {
    let mut digits = vec![];
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch.to_digit(10).unwrap());
        }
    }

    10 * digits.first().unwrap() + digits.last().unwrap()
}

fn get_calibration_value_v2(s: &str) -> u32 {
    let translate_table = HashMap::from([
        ("1", 1), ("one", 1),
        ("2", 2), ("two", 2),
        ("3", 3), ("three", 3),
        ("4", 4), ("four", 4),
        ("5", 5), ("five", 5),
        ("6", 6), ("six", 6),
        ("7", 7), ("seven", 7),
        ("8", 8), ("eight", 8),
        ("9", 9), ("nine", 9),
    ]);

    let mut base_idx = 0;
    let mut first = None;
    let mut last = None;

    while (first.is_none() || last.is_none()) && base_idx < s.len() {
        for kv in translate_table.iter() {
            if let None = first {
                let compare_window = &s[base_idx..min(base_idx + kv.0.len(), s.len())];

                if compare_window == *kv.0 {
                    first = Some(kv.1);
                }
            }

            if let None = last {
                let compare_window = &s[max(0, s.len().saturating_sub(kv.0.len()).saturating_sub(base_idx))..s.len().saturating_sub(base_idx)];

                if compare_window == *kv.0 {
                    last = Some(kv.1);
                }
            }
        }

        base_idx += 1;
    }

    10 * first.unwrap() + last.unwrap()
}

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut result: u32 = 0;

    for line in reader.lines() {
        result += get_calibration_value_v2(&line?);
    }

    println!("Calibration value of input is {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn get_calibration_value_v1_test() {
        assert_eq!(super::get_calibration_value_v1("1abc2"), 12);
        assert_eq!(super::get_calibration_value_v1("pqr3stu8vwx"), 38);
        assert_eq!(super::get_calibration_value_v1("a1b2c3d4e5f"), 15);
        assert_eq!(super::get_calibration_value_v1("treb7uchet"), 77);
    }

    #[test]
    fn get_calibration_value_v2_test() {
        assert_eq!(super::get_calibration_value_v2("1abc2"), 12);
        assert_eq!(super::get_calibration_value_v2("pqr3stu8vwx"), 38);
        assert_eq!(super::get_calibration_value_v2("a1b2c3d4e5f"), 15);
        assert_eq!(super::get_calibration_value_v2("treb7uchet"), 77);
        assert_eq!(super::get_calibration_value_v2("1abc2three"), 13);
        assert_eq!(super::get_calibration_value_v2("poneqr3stu8vwx"), 18);
        assert_eq!(super::get_calibration_value_v2("one"), 11);
        assert_eq!(super::get_calibration_value_v2("onetwothree"), 13);
        assert_eq!(super::get_calibration_value_v2("9onetwothree9"), 99);
        assert_eq!(super::get_calibration_value_v2("qgrgqjlszpcnpq82"), 82);
    }
}