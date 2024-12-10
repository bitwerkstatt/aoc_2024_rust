advent_of_code::solution!(8);

use pathfinding::matrix::Matrix;
use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Matrix<char> {
    let mut matrix = pathfinding::matrix::Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        ' ',
    );

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            matrix[(x, y)] = c;
        });
    });
    matrix
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(&input);

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..map.rows {
        for x in 0..map.columns {
            if map[(x, y)] != '.' {
                antennas
                    .entry(map[(x, y)])
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
    }

    for (_frequency, positions) in antennas {
        if positions.len() > 1 {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];
                    let distance: (isize, isize) =
                        (x2 as isize - x1 as isize, y2 as isize - y1 as isize);
                    if let Some((ax, ay)) =
                        map.move_in_direction((x1, y1), (distance.0 * 2, distance.1 * 2))
                    {
                        antinodes.insert((ax, ay));
                    }
                    if let Some((ax, ay)) =
                        map.move_in_direction((x2, y2), (distance.0 * -2, distance.1 * -2))
                    {
                        antinodes.insert((ax, ay));
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(&input);

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..map.rows {
        for x in 0..map.columns {
            if map[(x, y)] != '.' {
                antennas
                    .entry(map[(x, y)])
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
    }

    for (_frequency, positions) in antennas {
        if positions.len() > 1 {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];
                    let distance: (isize, isize) =
                        (x2 as isize - x1 as isize, y2 as isize - y1 as isize);
                    // starting at origin
                    let mut i = 1;
                    loop {
                        if let Some((ax, ay)) =
                            map.move_in_direction((x1, y1), (distance.0 * i, distance.1 * i))
                        {
                            antinodes.insert((ax, ay));
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    // starting at target
                    let mut i = 1;
                    loop {
                        if let Some((ax, ay)) =
                            map.move_in_direction((x2, y2), (distance.0 * -i, distance.1 * -i))
                        {
                            antinodes.insert((ax, ay));
                            i += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
