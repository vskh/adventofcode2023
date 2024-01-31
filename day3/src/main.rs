use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn is_part_number(schematic: &Vec<&str>, row: usize, range: RangeInclusive<usize>) -> bool {
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
            *range.start() - 1
        },
        if *range.end() < schematic[row].len() - 1 {
            range.end() + 1
        } else {
            schematic[row].len() - 1
        },
    );

    let num_str = &schematic[row][range.clone()];
    if !num_str.chars().all(|ch| ch.is_ascii_digit()) {
        return false;
    }

    for r in r_min..=r_max {
        if schematic[r][c_min..=c_max].chars().any(|ch| ch != '.' && !ch.is_ascii_digit()) {
            return true;
        }
    }

    false
}

fn calc_part_numbers_sum(schematic: &Vec<&str>) -> u64 {
    let mut part_numbers_sum = 0_u64;
    for (row, l) in schematic.iter().enumerate() {
        let mut part_start = -1_i32;
        for (idx, ch) in l.chars().enumerate() {
            if ch.is_ascii_digit() {
                if part_start == -1 {
                    part_start = idx as i32;
                }
            } else if part_start != -1 {
                let part_end = idx - 1;
                let number =  l[part_start as usize..=part_end].parse::<u64>().unwrap();
                let is_part = is_part_number(&schematic, row, part_start as usize..=part_end);

                #[cfg(debug_assertions)]
                println!("Found number '{}' in row {}, is_part = {}", number, row + 1, is_part);

                if is_part {
                    part_numbers_sum += number;
                }

                part_start = -1;
            }
        }
    }

    part_numbers_sum
}

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut schematic: Vec<String> = vec![];
    for line in reader.lines() {
        schematic.push(line.unwrap());
    }

    let sch = schematic.iter().map(AsRef::as_ref).collect::<Vec<&str>>();

    // search for part numbers
    let part_numbers_sum = calc_part_numbers_sum(&sch);

    println!("Sum of all part numbers in schematic is {}", part_numbers_sum);

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
        assert_eq!(super::is_part_number(&schematic, 5, 7..=8), false);

        assert_eq!(super::is_part_number(&schematic, 2, 2..=3), true);
        assert_eq!(super::is_part_number(&schematic, 7, 6..=8), true);
    }

    #[test]
    fn calc_part_numbers_sum_test() {
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

        assert_eq!(super::calc_part_numbers_sum(&schematic), 4361);
    }
}
