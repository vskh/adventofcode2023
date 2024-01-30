use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn is_part_number<T: AsRef<str>>(schematic: &Vec<T>, row: usize, range: RangeInclusive<usize>) -> bool {
    assert!(row < schematic.len());

    // this considers all lines in schematic to be of same length
    let (r_min, r_max) = (
        if row == 0 { 0 } else { row - 1 },
        if row < schematic.len() - 1 {
            row + 1
        } else {
            schematic.len() - 1
        },
    );
    let (c_min, c_max) = (
        if *range.start() == 0 {
            0
        } else {
            *range.start()
        },
        if *range.end() < schematic[row].len() - 1 {
            range.end() + 1
        } else {
            schematic[row].len()
        },
    );

    for r in r_min..=r_max {
        if schematic[r][c_min..=c_max].chars().any(|ch| ch != '.') {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut schematic: Vec<String> = vec![];
    for line in reader.lines() {
        schematic.push(line.unwrap());
    }

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn is_part_number_test() {
        let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(super::is_part_number(&schematic, 0, 5..=7), false);
        assert_eq!(super::is_part_number(&schematic, 5, 8..=9), false);
    }
}
