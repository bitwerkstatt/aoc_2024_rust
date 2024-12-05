advent_of_code::solution!(2);

fn check_sequence(sequence: &Vec<u32>) -> bool {
    let numbers = sequence;
    let mut last_diff: i32 = 0;
    let mut diff: i32 = 0;
    let mut valid: bool = true;
    for i in 1..numbers.len() {
        diff = numbers[i] as i32 - numbers[i - 1] as i32;
        if !(1..=3).contains(&diff.abs()) || (i > 1 && diff.signum() != last_diff.signum()) {
            valid = false;
        }
        last_diff = diff;
    }
    valid
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split(" ")
            .map(|s| s.parse().expect("Not a valid number"))
            .collect();
        if check_sequence(&numbers) {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split(" ")
            .map(|s| s.parse().expect("Not a valid number"))
            .collect();
        let mut valid: bool = false;
        // Brute force approach
        for i in 0..numbers.len() {
            let mut copy = numbers.clone();
            copy.remove(i);
            if check_sequence(&copy) {
                valid = true;
                break;
            }
        }
        if valid {
            result += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
