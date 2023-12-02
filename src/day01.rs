#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().map(|line|{
        let test:Vec<u32> = line.chars().filter(|x| x.is_numeric()).map(|x| x.to_digit(10).expect("")).collect();
        test[0]*10+test.last().expect("")
     }
    ).sum::<u32>() as i32
}

use std::collections::HashMap;

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let numbers: HashMap<&str, i32> = HashMap::from([
                                                ("one", 1),
                                                ("two", 2),
                                                ("three", 3),
                                                ("four", 4),
                                                ("five", 5),
                                                ("six", 6),
                                                ("seven", 7),
                                                ("eight", 8),
                                                ("nine", 9)
    ]);

    input.lines().map(|line| {
        let mut first: i32 = 0;
        let mut last: i32 = 0;
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                first = c.to_digit(10).unwrap() as i32;
                break;
            }
            for key in numbers.keys() {
                if line[0..i+1].contains(key) {
                    first = numbers[key];
                    break;
                }
            }
            if first != 0 {
                break;
            }
        }
        for (i, c) in line.chars().rev().enumerate() {
            if c.is_numeric() {
                last = c.to_digit(10).unwrap() as i32;
                break;
            }
            for key in numbers.keys() {

                if (&line[line.len()-i-1..]).contains(key) {
                    last = numbers[key];
                    break;
                }
            }
            if last != 0 {
                break;
            }
        }
        first*10 + last
    }).sum()
    
}
