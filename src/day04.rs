use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let numbers: Vec<&str> = input
        .lines()
        .map(|l| l.split(':').skip(1).next().unwrap())
        .collect();
    numbers
        .iter()
        .map(|l| {
            let mut result = 0;
            let line_split = l.split('|').collect::<Vec<&str>>();
            let winning_numbers: HashSet<i32> = HashSet::from_iter(
                line_split[0]
                    .split(' ')
                    .filter(|&x| !x.is_empty())
                    .map(|n| n.parse::<i32>().unwrap()),
            );

            line_split[1]
                .split(' ')
                .filter(|&x| !x.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .for_each(|n| {
                    if winning_numbers.contains(&n) {
                        result = match result {
                            0 => 1,
                            _ => result * 2,
                        };
                    }
                });
            result
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let numbers: Vec<&str> = input
        .lines()
        .map(|l| l.split(':').skip(1).next().unwrap())
        .collect();
    let mut ticket_counts = vec![1; numbers.len()];

    numbers
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let mut result = 0;
            let line_split = l.split('|').collect::<Vec<&str>>();
            let winning_numbers: HashSet<i32> = HashSet::from_iter(
                line_split[0]
                    .split(' ')
                    .filter(|&x| !x.is_empty())
                    .map(|n| n.parse::<i32>().unwrap()),
            );

            line_split[1]
                .split(' ')
                .filter(|&x| !x.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .for_each(|n| {
                    if winning_numbers.contains(&n) {
                        result += 1;
                    }
                });
            
            (i+1..=i+result).for_each(|j| {
                ticket_counts[j] += ticket_counts[i];
            });
            println!("line: {}, results: {}", i, result);
            result * ticket_counts[i]
        })
        .sum::<usize>();
        println!("{:?}", ticket_counts);
        ticket_counts.iter().sum()
}
