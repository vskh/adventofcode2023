use std::collections::{BTreeMap, HashMap, HashSet};
use std::{fs, io};

fn parse_card(line: &str) -> (u32, HashSet<u32>, HashSet<u32>) {
    let [id, content] = line.split(":").collect::<Vec<&str>>()[..] else {
        panic!(
            "Wrong card description, can't parse card id and content: {}",
            line
        );
    };
    let [_, card_id_str] = id.split_whitespace().collect::<Vec<&str>>()[..] else {
        panic!("Wrong card description, can't parse card id: {}", line);
    };
    let card_id = card_id_str.trim().parse::<u32>().unwrap();
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

    (card_id, winning_nums, available_nums)
}

fn calc_card_score(winning: &HashSet<u32>, available: &HashSet<u32>) -> u32 {
    let matching_nums_count = available.intersection(&winning).count();

    if matching_nums_count > 0 {
        2_u32.pow(matching_nums_count as u32 - 1)
    } else {
        0
    }
}

fn load_deck_from_file(file_path: &str) -> io::Result<BTreeMap<u32, (HashSet<u32>, HashSet<u32>)>> {
    Ok(load_deck_from_str(&fs::read_to_string(file_path)?))
}

fn load_deck_from_str(string: &str) -> BTreeMap<u32, (HashSet<u32>, HashSet<u32>)> {
    let mut result = BTreeMap::new();
    for card_desc in string.lines() {
        let (id, w, a) = parse_card(&card_desc);

        result.insert(id, (w, a));
    }

    result
}

fn calc_total_score(deck: &BTreeMap<u32, (HashSet<u32>, HashSet<u32>)>) -> u32 {
    deck.values().map(|(w, a)| calc_card_score(w, a)).sum()
}

fn calc_total_cards(deck: &BTreeMap<u32, (HashSet<u32>, HashSet<u32>)>) -> u32 {
    let mut collected_cards = HashMap::new();
    let mut calc_queue = Vec::from_iter(deck.keys().map(|&k| k));

    while let Some(card) = calc_queue.pop() {
        match collected_cards.get_mut(&card) {
            Some(count) => {
                *count += 1;
            }
            None => {
                collected_cards.insert(card, 1);
            }
        }

        let (w, a) = deck.get(&card).unwrap();
        let score = a.intersection(w).count();

        calc_queue.append(&mut ((card + 1)..(card + 1 + score as u32)).collect::<Vec<u32>>());
    }

    collected_cards.values().sum()
}

fn main() -> io::Result<()> {
    let deck = load_deck_from_file("input.txt")?;

    let total_score: u32 = calc_total_score(&deck);
    let total_cards: u32 = calc_total_cards(&deck);

    println!("Total cards score is {}", total_score);
    println!("Total number of collected cards is {}", total_cards);

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, HashSet};

    #[test]
    fn parse_card_test() {
        assert_eq!(
            super::parse_card("Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            (
                1,
                HashSet::from([41, 48, 83, 86, 17]),
                HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
            )
        );
    }

    #[test]
    fn calc_card_score() {
        assert_eq!(
            super::calc_card_score(&HashSet::from([1, 2, 3]), &HashSet::from([4, 5, 6])),
            0
        );
        assert_eq!(
            super::calc_card_score(&HashSet::from([1, 2, 3]), &HashSet::from([1, 5, 6])),
            1
        );
        assert_eq!(
            super::calc_card_score(&HashSet::from([1, 2, 3]), &HashSet::from([1, 2, 6])),
            2
        );
        assert_eq!(
            super::calc_card_score(&HashSet::from([1, 2, 3]), &HashSet::from([1, 2, 3])),
            4
        );
        assert_eq!(
            super::calc_card_score(&HashSet::from([1, 2, 3, 4]), &HashSet::from([1, 2, 3, 4])),
            8
        );
    }

    #[test]
    fn load_deck_from_str_test() {
        let cards = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";

        assert_eq!(
            super::load_deck_from_str(cards),
            BTreeMap::from([
                (
                    1,
                    (
                        HashSet::from([41, 48, 83, 86, 17]),
                        HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
                    )
                ),
                (
                    2,
                    (
                        HashSet::from([13, 32, 20, 16, 61]),
                        HashSet::from([61, 30, 68, 82, 17, 32, 24, 19])
                    )
                )
            ])
        );
    }

    #[test]
    fn calc_total_cards() {
        let cards = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(
            super::calc_total_cards(&super::load_deck_from_str(cards)),
            30
        );
    }
}
