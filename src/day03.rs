use std::ops::RangeInclusive;

#[derive(Debug)]
struct NumberRange {
    value: i32,
    h_range: RangeInclusive<i32>,
    v_range: RangeInclusive<i32>,
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let specials: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| !c.is_numeric() && c != '.').collect())
        .collect();

    let numbers: Vec<Vec<NumberRange>> = input
        .lines()
        .enumerate()
        .map(|(j, l)| {
            let j = j as i32;
            let mut number_collector = String::from("");
            l.chars()
                .enumerate()
                .map(|(i, c)| {
                    let i = i as i32;
                    if c.is_numeric() {
                        number_collector.push(c);
                        if i + 1 == l.len() as i32 {
                            return Some(NumberRange {
                                value: number_collector.parse().unwrap(),
                                h_range: i - 1 - number_collector.len() as i32..=i,
                                v_range: j - 1..=j + 1,
                            });
                        }
                    } else if !number_collector.is_empty() {
                        let number_lenght = number_collector.len() as i32;
                        let h_start = if i - 1 - number_lenght > 0 {
                            i - 1 - number_lenght
                        } else {
                            0
                        };
                        let v_start = if j - 1 > 0 { j - 1 } else { 0 };
                        let v_end = if (j + 1) < specials.len() as i32 {
                            j + 1
                        } else {
                            j
                        };

                        let num = NumberRange {
                            value: number_collector.parse().unwrap(),
                            h_range: h_start..=i,
                            v_range: v_start..=v_end,
                        };
                        number_collector = String::from("");
                        return Some(num);
                    }
                    return None;
                })
                .flatten()
                .collect()
        })
        .collect();

    numbers
        .iter()
        .map(|nums| {
            nums.iter()
                .map(|n| {
                    for (i, j) in itertools::iproduct!(n.h_range.clone(), n.v_range.clone()) {
                        if specials[j as usize][i as usize] {
                            return n.value;
                        }
                    }
                    0
                })
                .sum::<i32>()
        })
        .sum()
}

#[derive(Debug)]
struct SpecialCount {
    value: i32,
    count: usize,
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut specials: Vec<Vec<Option<SpecialCount>>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if !c.is_numeric() && c != '.' {
                        Some(SpecialCount { value:1, count: 0 })
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let numbers: Vec<Vec<NumberRange>> = input
        .lines()
        .enumerate()
        .map(|(j, l)| {
            let j = j as i32;
            let mut number_collector = String::from("");
            l.chars()
                .enumerate()
                .map(|(i, c)| {
                    let i = i as i32;
                    if c.is_numeric() {
                        number_collector.push(c);
                        if i + 1 == l.len() as i32 {
                            return Some(NumberRange {
                                value: number_collector.parse().unwrap(),
                                h_range: i - 1 - number_collector.len() as i32..=i,
                                v_range: j - 1..=j + 1,
                            });
                        }
                    } else if !number_collector.is_empty() {
                        let number_lenght = number_collector.len() as i32;
                        let h_start = if i - 1 - number_lenght > 0 {
                            i - 1 - number_lenght
                        } else {
                            0
                        };
                        let v_start = if j - 1 > 0 { j - 1 } else { 0 };
                        let v_end = if (j + 1) < specials.len() as i32 {
                            j + 1
                        } else {
                            j
                        };

                        let num = NumberRange {
                            value: number_collector.parse().unwrap(),
                            h_range: h_start..=i,
                            v_range: v_start..=v_end,
                        };
                        number_collector = String::from("");
                        return Some(num);
                    }
                    return None;
                })
                .flatten()
                .collect()
        })
        .collect();

    numbers.iter().for_each(|nums| {
        nums.iter().for_each(|n| {
            for (i, j) in itertools::iproduct!(n.h_range.clone(), n.v_range.clone()) {
                match specials[j as usize][i as usize].as_mut() {
                    Some(s) => {
                        s.count += 1;
                        s.value *= n.value;
                    }
                    None => (),
                }
            }
        });
    });
    specials
        .iter()
        .map(|l| {
            l.iter().map(|s| match s {
                Some(i) => {
                    if i.count == 2 {
                        i.value
                    } else {
                        0
                    }
                }
                None => 0,
            }).sum::<i32>()
        })
        .sum::<i32>()
}
