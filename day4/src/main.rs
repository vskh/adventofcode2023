use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn parse_card(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let [_id, content] = line.split(":").collect::<Vec<&str>>()[..] else {
        panic!("Wrong card description, can't get card id: {}", line);
    };
    let [winning, available] = content.split("|").collect::<Vec<&str>>()[..] else {
        panic!(
            "Wrong card description, can't figure winning numbers from available: {}",
            line
        )
    };

    let winning_nums = winning
        .split_whitespace()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    let available_nums = available
        .split_whitespace()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();

    (winning_nums, available_nums)
}

fn calc_card_score(winning: HashSet<u32>, available: HashSet<u32>) -> u32 {
    let matching_nums_count = available.intersection(&winning).count();

    if matching_nums_count > 0 {
        2_u32.pow(matching_nums_count as u32 - 1)
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut total_score = 0;
    for l in reader.lines() {
        if let Ok(card_desc) = l {
            let (w, a) = parse_card(&card_desc);
            let score = calc_card_score(w, a);

            total_score += score;
        }
    }

    println!("Total cards score is {}", total_score);

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn parse_card_test() {
        assert_eq!(
            super::parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            (
                HashSet::from([41, 48, 83, 86, 17]),
                HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
            )
        );
    }

    #[test]
    fn calc_card_score() {
        assert_eq!(super::calc_card_score(HashSet::from([1, 2, 3]), HashSet::from([4, 5, 6])), 0);
        assert_eq!(super::calc_card_score(HashSet::from([1, 2, 3]), HashSet::from([1, 5, 6])), 1);
        assert_eq!(super::calc_card_score(HashSet::from([1, 2, 3]), HashSet::from([1, 2, 6])), 2);
        assert_eq!(super::calc_card_score(HashSet::from([1, 2, 3]), HashSet::from([1, 2, 3])), 4);
        assert_eq!(super::calc_card_score(HashSet::from([1, 2, 3, 4]), HashSet::from([1, 2, 3, 4])), 8);
    }
}
