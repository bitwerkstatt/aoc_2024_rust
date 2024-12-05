advent_of_code::solution!(4);

fn parse(input: &str) -> pathfinding::matrix::Matrix<char> {
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
    let needle: String = String::from("XMAS");
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];
    let mut result: u32 = 0;
    let matrix = parse(input);
    for x in 0..matrix.columns {
        for y in 0..matrix.rows {
            if matrix[(x, y)] == 'X' {
                for dir in &directions {
                    'direction: for i in 1..4 {
                        let (dx, dy) = dir;
                        let nx = x as isize + dx * i as isize;
                        let ny = y as isize + dy * i as isize;
                        if nx < 0
                            || ny < 0
                            || nx >= matrix.columns as isize
                            || ny >= matrix.rows as isize
                        {
                            break 'direction;
                        }
                        let cand = matrix[(nx as usize, ny as usize)];

                        if cand != needle.chars().nth(i).unwrap() {
                            break 'direction;
                        }
                        if i == 3 {
                            result += 1;
                        }
                    }
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let matrix = parse(input);
    for x in 1..matrix.columns - 1 {
        for y in 1..matrix.rows - 1 {
            if matrix[(x, y)] == 'A' {
                if ((matrix[(x - 1, y - 1)] == 'M')
                    && (matrix[(x + 1, y + 1)] == 'S')
                    && (matrix[(x - 1, y + 1)] == 'M')
                    && (matrix[(x + 1, y - 1)] == 'S'))
                    || ((matrix[(x - 1, y - 1)] == 'S')
                        && (matrix[(x + 1, y + 1)] == 'M')
                        && (matrix[(x - 1, y + 1)] == 'S')
                        && (matrix[(x + 1, y - 1)] == 'M'))
                    || ((matrix[(x - 1, y - 1)] == 'M')
                        && (matrix[(x + 1, y + 1)] == 'S')
                        && (matrix[(x + 1, y - 1)] == 'M')
                        && (matrix[(x - 1, y + 1)] == 'S'))
                    || ((matrix[(x - 1, y - 1)] == 'S')
                        && (matrix[(x + 1, y + 1)] == 'M')
                        && (matrix[(x + 1, y - 1)] == 'S')
                        && (matrix[(x - 1, y + 1)] == 'M'))
                {
                    result += 1;
                }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
