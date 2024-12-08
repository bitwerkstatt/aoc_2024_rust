use std::collections::HashSet;
use itertools::Itertools;
use pathfinding::matrix::Matrix;

advent_of_code::solution!(6);

enum SimState {
    Initialized,
    Running,
    GuardExited(usize),
    GuardLooped
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}
#[derive(Clone, Debug, PartialEq)]
enum GuardState {
    Walking,
    OffDuty(usize),
    Looping
}

#[derive(Clone, Debug)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
    visited: HashSet<(usize, usize, Direction)>,
    state: GuardState
}

impl Guard {
    fn new(x: usize, y: usize) -> Guard {
        Guard {
            x,
            y,
            direction: Direction::Up,
            visited: HashSet::new(),
            state: GuardState::Walking
        }
    }

    fn do_move(&mut self, map: &Matrix<char>) -> &GuardState {
        let new_pos: (isize, isize);
        let new_dir: Direction;
        match self.direction {
            Direction::Up => {
                new_pos = (self.x as isize, self.y as isize - 1);
                new_dir = Direction::Right
            },
            Direction::Right => {
                new_pos = ((self.x + 1) as isize, self.y as isize);
                new_dir = Direction::Down
            },
            Direction::Down => {
                new_pos = (self.x as isize, (self.y + 1) as isize);
                new_dir = Direction::Left
            },
            Direction::Left => {
                new_pos = (self.x as isize - 1, self.y as isize);
                new_dir = Direction::Up
            },
        }
        if new_pos.0 >=0 && new_pos.0 < map.columns as isize && new_pos.1 >= 0 && new_pos.1 < map.rows as isize {
           if map[(new_pos.0 as usize, new_pos.1 as usize)] != '#' {
               // step forward
               self.x = new_pos.0 as usize;
               self.y = new_pos.1 as usize;
           } else {
               self.direction = new_dir;
           }
           // Check for a loop
           if self.visited.contains(&(self.x, self.y, self.direction.clone())) {
               self.state = GuardState::Looping;
           }

           self.visited.insert((self.x, self.y, self.direction.clone()));
        } else {
            let fields_visited: HashSet<(usize, usize)> = self.visited.iter()
                .map(|(x, y, _)| (*x, *y))
                .collect();
            self.state = GuardState::OffDuty(fields_visited.len())
        }

        &self.state
    }

}

struct GuardSimulation {
    map: Matrix<char>,
    guard: Guard,
    state: SimState
}

impl GuardSimulation {

    pub fn new(map: Matrix<char>, guard: Guard) -> GuardSimulation {
        GuardSimulation {
            map,
            guard,
            state: SimState::Initialized
        }
    }

    fn step(&mut self) {

        let new_guard_state = self.guard.do_move(&self.map);
        match new_guard_state {
            GuardState::Walking => { self.state = SimState::Running}
            GuardState::OffDuty(visited_fields) => {
                self.state = SimState::GuardExited(*visited_fields);
            }
            GuardState::Looping => {
                self.state = SimState::GuardLooped
            }
        }
    }
}

fn parse(input: &str) -> (Matrix<char>, (usize,usize)) {
    let mut matrix = Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        ' ',
    );

    let mut start_pos: (usize, usize) = (0,0);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            matrix[(x, y)] = c;
            if c == '^' {
                start_pos = (x ,y);
            }
        });
    });
    (matrix, start_pos)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let (matrix, start_pos) = parse(input);
    let guard = Guard::new(start_pos.0, start_pos.1);
    let mut sim: GuardSimulation = GuardSimulation::new(matrix, guard);
    loop {
        sim.step();
        match sim.state {
            SimState::Running => {},
            SimState::GuardExited(fields_visited) => {
                result = fields_visited as u32;
                break;
            },
            _ => {
                // Don't care for now
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let (mut matrix, start_pos) = parse(input);
    let guard = Guard::new(start_pos.0, start_pos.1);

/*    matrix[(3,6)] = '#';
    let sim_guard = guard.clone();
    let mut simulation = GuardSimulation::new(matrix, sim_guard);
    loop {
        simulation.step();
        match simulation.state {
            SimState::Running => {},
            SimState::GuardLooped => {
                result += 1;
                break;
            },
            _ => {
                // Don't care for now
            }
        }
    }*/
    for x in 0..matrix.rows {
        for y in 0.. matrix.columns {
            if (x,y) != start_pos && matrix[(x,y)] != '#'  {
                let mut candidate_matrix = matrix.clone();
                candidate_matrix[(x,y)]='#';
                let sim_guard = guard.clone();
                let mut simulation = GuardSimulation::new(candidate_matrix, sim_guard);
                'simulation: loop {
                    simulation.step();
                    match simulation.state {
                        SimState::Running => {},
                        SimState::GuardExited(fields_visited) => {
                            break;
                        }
                        SimState::GuardLooped => {
                            result += 1;
                            break 'simulation;
                        },
                        _ => {
                            // Don't care for now
                        }
                    }
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
