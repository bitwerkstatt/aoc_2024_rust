advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let re = Regex::new(r"mul\((?<f1>\d{1,3}),(?<f2>\d{1,3})\)").unwrap();

    let multiplications : Vec<(u32, u32)> = re.captures_iter(input).map(|caps| {
        let f1 = caps.name("f1").unwrap().as_str().parse().expect("Not a valid number");
        let f2 = caps.name("f2").unwrap().as_str().parse().expect("Not a valid number");
        (f1, f2)
    }).collect();

    for (f1, f2) in multiplications {
        result += f1*f2;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let re_multi = Regex::new(r"(?<cnd>don't|do)|mul\((?<f1>\d{1,3}),(?<f2>\d{1,3})\)").unwrap();
    let mut enabled: bool = true;
    let mut multiplications: Vec<(u32, u32)> = Vec::new();
    for caps in re_multi.captures_iter(input) {
        if let Some(cnd) = caps.name("cnd") {
            if cnd.as_str() == "don't" {
                enabled = false;
            } else {
                enabled = true;
            }
        } else if let (Some(f1), Some(f2)) = (caps.name("f1"), caps.name("f2")) {
            let f1: u32 = f1.as_str().parse().expect("Not a valid number");
            let f2: u32 = f2.as_str().parse().expect("Not a valid number");
            if enabled {
                multiplications.push((f1, f2));
            }
        }
    }

    for (f1, f2) in multiplications {
        result += f1*f2;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
