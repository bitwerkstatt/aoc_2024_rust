advent_of_code::solution!(7);

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    let mut result:Vec<(usize, Vec<usize>)> = Vec::new();

    for line in input.lines() {
        let (first, rest) = line.split_once(": ").unwrap();
        let operands = rest.split_whitespace().map(|o| o.parse().unwrap()).collect();
        let value: usize = first.parse().unwrap();
        result.push((value ,operands));
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut result: usize = 0;
    let equations = parse(input);

    for (value, operands) in equations {
    let combinations = 2u32.pow((operands.len() - 1) as u32);
    for mut i in 0..combinations {
        let mut iter = operands.iter();
        let mut test = *iter.next().unwrap();
        for &number in iter {
            test = if i & 1 == 0 { test + number } else { test * number };
            i >>= 1;
        }
        if test == value {
            result += value;
            break;
        }
    }
}

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result: usize = 0;
    let equations = parse(input);

    for (value, operands) in equations {
        let combinations = 3u32.pow((operands.len() - 1) as u32);
        for mut i in 0..combinations {
            let mut iter = operands.iter();
            let mut test = *iter.next().unwrap();
            for &number in iter {
                if i % 3 == 0 {
                    test += number;
                } else if i % 3 == 1 {
                    test *= number;
                } else {
                    test *= 10u64.pow(number.ilog10() + 1) as usize;
                    test += number;
                }
                i /= 3;
            }
            if test == value {
                result += value;
                break;
            }
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
