use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn extract_number_at_pos(line: &str, pos: usize) -> Option<(u64, (usize, usize))> {
    let line_as_chars = line.chars().collect::<Vec<char>>();
    if pos > line_as_chars.len() || !line_as_chars[pos].is_ascii_digit() {
        None
    } else {
        let mut num_start = pos;
        let mut num_end = pos;

        while num_start != 0 {
            if line_as_chars[num_start - 1].is_ascii_digit() {
                num_start -= 1;
            } else {
                break;
            }
        }

        while num_end < line_as_chars.len() - 1 {
            if line_as_chars[num_end + 1].is_ascii_digit() {
                num_end += 1;
            } else {
                break;
            }
        }

        let num_str = &line[num_start..=num_end];
        let num = num_str.parse::<u64>().unwrap();

        Some((num, (num_start, num_end)))
    }
}

fn calc_gear_ratio(schematic: &Vec<&str>, row: usize, col: usize) -> Option<u64> {
    if row > schematic.len()
        || col > schematic[row].len()
        || schematic[row].chars().nth(col).unwrap() != '*'
    {
        None
    } else {
        let min_row = if row == 0 { row } else { row - 1 };
        let max_row = if row == schematic.len() - 1 {
            row
        } else {
            row + 1
        };
        let min_col = if col == 0 { col } else { col - 1 };
        let max_col = if col == schematic[row].len() - 1 {
            col
        } else {
            col + 1
        };

        let mut adjacent_nums = HashSet::new();
        for r in min_row..=max_row {
            for c in min_col..=max_col {
                if let Some(n) = extract_number_at_pos(&schematic[r], c) {
                    adjacent_nums.insert(n);
                }
            }
        }

        if adjacent_nums.len() != 2 {
            None
        } else {
            Some(adjacent_nums.iter().fold(1, |acc, el| acc * el.0))
        }
    }
}

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
        if schematic[r][c_min..=c_max]
            .chars()
            .any(|ch| ch != '.' && !ch.is_ascii_digit())
        {
            return true;
        }
    }

    false
}

fn calc_part_numbers_sum_and_gear_ratios_sum(schematic: &Vec<&str>) -> (u64, u64) {
    fn process_number(schematic: &Vec<&str>, row: usize, start: usize, end: usize) -> u64 {
        let number = schematic[row][start..=end].parse::<u64>().unwrap();
        let is_part = is_part_number(&schematic, row, start..=end);

        #[cfg(debug_assertions)]
        println!(
            "Found number '{}' in row {}, is_part = {}",
            number,
            row + 1,
            is_part
        );

        if is_part {
            number
        } else {
            0
        }
    }

    let mut part_numbers_sum = 0_u64;
    let mut gear_ratios_sum = 0_u64;
    for (row, l) in schematic.iter().enumerate() {
        let mut part_start = -1_i32;
        for (col, ch) in l.chars().enumerate() {
            if ch == '*' {
                if let Some(r) = calc_gear_ratio(schematic, row, col) {
                    gear_ratios_sum += r;
                }
            }

            if ch.is_ascii_digit() {
                if part_start == -1 {
                    part_start = col as i32;
                }
            } else if part_start != -1 {
                let part_end = col - 1;

                part_numbers_sum += process_number(schematic, row, part_start as usize, part_end);

                part_start = -1;
            }
        }

        if part_start != -1 {
            let part_end = l.len() - 1;

            part_numbers_sum += process_number(schematic, row, part_start as usize, part_end);
        }
    }

    (part_numbers_sum, gear_ratios_sum)
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
    let (part_numbers_sum, gear_ratios_sum) = calc_part_numbers_sum_and_gear_ratios_sum(&sch);

    println!(
        "Sum of all part numbers in schematic is {}",
        part_numbers_sum
    );
    println!("Sum of all gear ratios in schematic is {}", gear_ratios_sum);

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
    fn calc_part_numbers_sum_and_gear_ratios_sum_test() {
        let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*...&
.664.598.1"
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(
            super::calc_part_numbers_sum_and_gear_ratios_sum(&schematic),
            (4362, 467835)
        );
    }

    #[test]
    fn extract_number_at_pos_test() {
        assert_eq!(super::extract_number_at_pos("...", 1), None);
        assert_eq!(super::extract_number_at_pos(".1.", 1), Some((1, (1, 1))));
        assert_eq!(
            super::extract_number_at_pos(".123.", 1),
            Some((123, (1, 3)))
        );
        assert_eq!(
            super::extract_number_at_pos(".123.", 2),
            Some((123, (1, 3)))
        );
        assert_eq!(
            super::extract_number_at_pos(".123.", 3),
            Some((123, (1, 3)))
        );
        assert_eq!(super::extract_number_at_pos(".123.", 4), None);
    }

    #[test]
    fn calc_gear_ratio() {
        let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*...&
.664.598.1"
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(super::calc_gear_ratio(&schematic, 1, 3), Some(16345));
        assert_eq!(super::calc_gear_ratio(&schematic, 8, 5), Some(451490));
        assert_eq!(super::calc_gear_ratio(&schematic, 4, 3), None);
    }
}
