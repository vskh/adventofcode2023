use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn get_calibration_value(s: &str) -> u32 {
    let mut digits = vec![];
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch.to_digit(10).unwrap());
        }
    }

    10 * digits.first().unwrap() + digits.last().unwrap()
}

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut result: u32 = 0;

    for line in reader.lines() {
        result += get_calibration_value(&line?);
    }

    println!("Calibration value of input is {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn get_calibration_value_test() {
        assert_eq!(super::get_calibration_value("1abc2"), 12);
        assert_eq!(super::get_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(super::get_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(super::get_calibration_value("treb7uchet"), 77);
    }
}