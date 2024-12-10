use std::collections::HashSet;

advent_of_code::solution!(5);

fn parse(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let rules: HashSet<(u32, u32)> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            line.split_once('|')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        })
        .collect();

    let mut updates: Vec<Vec<u32>> = Vec::new();
    let update_lines: Vec<&str> = input.lines().skip(rules.len() + 1).collect();

    for line in update_lines {
        let mut update: Vec<u32> = Vec::new();
        let elements = line.split(",");
        for element in elements {
            update.push(element.parse().unwrap())
        }
        updates.push(update);
    }

    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let (rules, updates) = parse(input);

    for update in updates {
        let mut valid: bool = true;
        'outer: for i in 0..update.len() {
            for j in i + 1..update.len() {
                if !rules.contains(&(update[i], update[j])) {
                    valid = false;
                    break 'outer;
                }
            }
        }
        if valid {
            result += update[update.len() / 2];
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let (rules, updates) = parse(input);

    for mut update in updates {
        let mut valid: bool = true;
        'outer: for i in 0..update.len() {
            for j in i + 1..update.len() {
                if !rules.contains(&(update[i], update[j])) {
                    valid = false;
                    break 'outer;
                }
            }
        }
        if !valid {
            update.sort_by(|i, j| {
                if rules.contains(&(*i, *j)) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            result += update[update.len() / 2];
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
