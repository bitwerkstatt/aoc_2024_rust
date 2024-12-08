use std::collections::HashSet;
use pathfinding::matrix::Matrix;

advent_of_code::solution!(6);


fn parse(input: &str) -> (Matrix<char>, (isize,isize)) {
    let mut matrix = Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        ' ',
    );

    let mut start_pos: (isize, isize) = (0,0);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            matrix[(x, y)] = c;
            if c == '^' {
                start_pos = (x as isize,y as isize);
            }
        });
    });
    (matrix, start_pos)
}

fn check_bounds(pos: &(isize, isize, usize), map: &Matrix<char>) -> bool {
    let width = map.columns as isize;
    let height =map.columns as isize;
    pos.0 >= 0 && pos.0 < width && pos.1 >=0 && pos.1 <height
}



pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let (matrix, start_pos) = parse(input);
    let mut pos: (isize, isize, usize) = (start_pos.0, start_pos.1, 0); //third component is direction: 0=north, 1, = east a.s.o.
    let mut visited: HashSet<(isize, isize, usize)> = HashSet::new();
    loop {
        visited.insert(pos);
        let mut new_pos: (isize, isize, usize);
        match pos.2 {
            0 => {
                new_pos = (pos.0, pos.1 - 1, pos.2);
            },
            1 => {
                new_pos = (pos.0 + 1, pos.1, pos.2);
            },
            2 => {
                new_pos = (pos.0, pos.1 + 1, pos.2);
            },
            3 => {
                new_pos = (pos.0-1, pos.1, pos.2);
            },
            _ => {panic!("Unknown direction!")}
        }

        if !check_bounds(&new_pos, &matrix) {
            break;
        }

        if matrix[(new_pos.0 as usize, new_pos.1 as usize)]!='#' {
                pos = new_pos;
        } else {
            pos = (pos.0, pos.1, (pos.2+1)%4);
        }
    }

    let reduced_visited: HashSet<(isize, isize)> = visited.iter()
        .map(|(x, y, _)| (*x, *y))
        .collect();
    result = reduced_visited.len() as u32;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let (matrix, start_pos) = parse(input);
    let mut pos: (isize, isize, usize) = (start_pos.0, start_pos.1, 0); //third component is direction: 0=north, 1, = east a.s.o.
    let mut visited: HashSet<(isize, isize, usize)> = HashSet::new();
    loop {
        visited.insert(pos);
        let mut new_pos: (isize, isize, usize);
        match pos.2 {
            0 => {
                new_pos = (pos.0, pos.1 - 1, pos.2);
            },
            1 => {
                new_pos = (pos.0 + 1, pos.1, pos.2);
            },
            2 => {
                new_pos = (pos.0, pos.1 + 1, pos.2);
            },
            3 => {
                new_pos = (pos.0-1, pos.1, pos.2);
            },
            _ => {panic!("Unknown direction!")}
        }

        if !check_bounds(&new_pos, &matrix) {
            break;
        }

        if matrix[(new_pos.0 as usize, new_pos.1 as usize)]!='#' {
            pos = new_pos;
            // Simulation if there was an obstacle
            // Walk until OOB or on a visited spot with the same direction
            let mut sim_pos = (pos.0, pos.1, (pos.2+1)%4);
            'sim: loop {
                match sim_pos.2 {
                    0 => {
                        sim_pos = (sim_pos.0, sim_pos.1 - 1, sim_pos.2 );
                    },
                    1 => {
                        sim_pos = (sim_pos.0 + 1, sim_pos.1, sim_pos.2 );
                    },
                    2 => {
                        sim_pos = (sim_pos.0, sim_pos.1 + 1, sim_pos.2 );
                    },
                    3 => {
                        sim_pos = (sim_pos.0 - 1 , sim_pos.1, sim_pos.2 );
                    },
                    _ => panic!("Unknown direction!")
                }
                if visited.contains(&sim_pos) {
                    result += 1;
                    break 'sim;
                }
                if !check_bounds(&sim_pos, &matrix) {
                    break 'sim;
                }
            }
        } else {
            pos = (pos.0, pos.1, (pos.2+1)%4);
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
