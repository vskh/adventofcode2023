use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn parse_game_id(game_name: &str) -> u32 {
    let game_id = game_name.strip_prefix("Game ").unwrap();

    game_id.parse().unwrap()
}

fn parse_game_turn(turn: &str) -> [u8; 3] {
    turn.split(",").map(|s| s.trim()).fold([0, 0, 0], |acc, cubes_desc| {
        let [r, g, b] = acc;

        let [amount, color] = cubes_desc.split(" ").collect::<Vec<&str>>()[..] else {panic!("Bad game turn line: {}", turn)};

        match color {
            "red" => [amount.parse().unwrap(), g, b],
            "green" => [r, amount.parse().unwrap(), b],
            "blue" => [r, g, amount.parse().unwrap()],
            _ => panic!("Bad cubes color {} in turn line '{}'", color, turn)
        }
    })
}

fn parse_game_turns(content: &str) -> Vec<[u8; 3]> {
    content.split(";").map(|s| s.trim()).map(parse_game_turn).collect()
}

fn parse_game(desc: &str) -> (u32, Vec<[u8; 3]>)
{
    let [name, content] = desc.split(":").collect::<Vec<&str>>()[..] else { panic!("Bad game line: {}", desc) };

    (parse_game_id(name), parse_game_turns(content))
}

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    const MAX_CUBES: [u8; 3] = [12, 13, 14];

    let mut possible_game_ids_sum = 0;
    for line in reader.lines() {
        let (game_id, game_turns) = parse_game(&line?);
        let is_possible = game_turns.iter().all(|turn| turn[0] <= MAX_CUBES[0] && turn[1] <= MAX_CUBES[1] && turn[2] <= MAX_CUBES[2]);

        if is_possible {
            possible_game_ids_sum += game_id;
        }
    }

    println!("Possible game IDs sum is {}", possible_game_ids_sum);

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_game_test() {
        assert_eq!(super::parse_game("Game 1: 1 red"), (1, vec![[1, 0, 0]]));
    }

    #[test]
    fn parse_game_id_test() {
        assert_eq!(super::parse_game_id("Game 11"), 11);
    }

    #[test]
    fn parse_game_turns_test() {
        assert_eq!(super::parse_game_turns("1 green; 2 red; 3 blue"), vec![[0, 1, 0], [2, 0, 0], [0, 0, 3]]);
    }

    #[test]
    fn parse_game_turn_ok_test() {
        assert_eq!(super::parse_game_turn("1 red, 2 green, 3 blue"), [1, 2, 3]);
        assert_eq!(super::parse_game_turn("2 green, 3 blue"), [0, 2, 3]);
        assert_eq!(super::parse_game_turn("1 red, 3 blue"), [1, 0, 3]);
        assert_eq!(super::parse_game_turn("1 red, 2 green"), [1, 2, 0]);
    }

    #[test]
    #[should_panic]
    fn parse_game_turn_bad_format_test() {
        super::parse_game_turn("ai karramba lol,");
    }

    #[test]
    #[should_panic]
    fn parse_game_turn_bad_color_test() {
        super::parse_game_turn("1 violet, 2 magenta, 3 purple");
    }
}
