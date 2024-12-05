advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse().expect("Not a valid number"));
        right.push(parts[1].parse().expect("Not a valid number"));
    }
    left.sort();
    right.sort();
    for (l, r) in left.iter().zip(right.iter()) {
        result += (l - r).abs() as u32;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse().expect("Not a valid number"));
        right.push(parts[1].parse().expect("Not a valid number"));
    }

    for &number in &left {
        let occurences = right.iter().filter(|&&n| n == number).count() as u32;
        result += number as u32 * occurences;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
