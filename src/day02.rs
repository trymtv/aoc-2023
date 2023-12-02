use itertools::Itertools;

fn is_game_allowed(number: i32, color: &str) -> bool {
    let allowed_number = match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => panic!(),
    };
    allowed_number >= number
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let valid_ids = input.lines().map(|line| {
        let (game_id, game_content) = line.split(":").collect_tuple().unwrap();
        for game in game_content.split(";") {
            for dice in game.split(",") {
                let (dice_number, dice_color): (&str, &str) =
                    dice.split_whitespace().collect_tuple().unwrap();
                if !is_game_allowed(dice_number.parse::<i32>().unwrap(), dice_color) {
                    return 0;
                }
            }
        }
        game_id.split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap()
    });
    valid_ids.sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let valid_ids = input.lines().map(|line| {
        let mut highest_red = 0;
        let mut highest_green = 0;
        let mut highest_blue = 0;
        let (_, game_content) = line.split(":").collect_tuple().unwrap();
        for game in game_content.split(";") {
            for dice in game.split(",") {
                let (dice_number, dice_color): (&str, &str) =
                    dice.split_whitespace().collect_tuple().unwrap();
                let dice_number = dice_number.parse::<i32>().unwrap();
                match dice_color {
                    "red" => {
                        if dice_number > highest_red {
                            highest_red = dice_number
                        }
                    }
                    "green" => {
                        if dice_number > highest_green {
                            highest_green = dice_number
                        }
                    }
                    "blue" => {
                        if dice_number > highest_blue {
                            highest_blue = dice_number
                        }
                    }
                    _ => panic!(),
                };
            }
        }
        highest_red * highest_blue * highest_green
    });
    valid_ids.sum()
}
