use std::collections::HashSet;

pub type Position = (isize, isize);
pub type History = HashSet<Position>;

#[derive (Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn new_position(state: &mut (Position, Direction), instruction: &str, history: &mut History) {
    if let Ok(new_direction) = turn(state.1, &instruction[0..1]) {
        state.1 = new_direction;
        move_position(state, isize::from_str_radix(&instruction[1..instruction.len()], 10).unwrap(), history);
    } else {
        eprintln!("Could not calculate new position");
    }
}

fn turn(direction: Direction, instruction: &str) -> Result<Direction, &str> {
    match instruction {
        "L" => {
            match direction {
                Direction::North => {
                    return Ok(Direction::West);
                },
                Direction::East => {
                    return Ok(Direction::North);
                },
                Direction::South => {
                    return Ok(Direction::East);
                },
                Direction::West => {
                    return Ok(Direction::South);
                }
            }
        },
        "R" => {
            match direction {
                Direction::North => {
                    return Ok(Direction::East);
                },
                Direction::East => {
                    return Ok(Direction::South);
                },
                Direction::South => {
                    return Ok(Direction::West);
                },
                Direction::West => {
                    return Ok(Direction::North);
                }
            }
        },
        _ => {
            return Err("Something was passed that is not an L or an R.");
        }
    }
}

fn move_position(state: &mut (Position, Direction), distance: isize, history: &mut History) {
    match state.1 {
        Direction::North => {
            for _i in 0..distance {
                state.0.1 += 1;
                if !history.insert(state.0) {
                    println!("We've been here before, distance = {}", distance_to_origin(&state.0));
                }
            }
        },
        Direction::East => {
            for _i in 0..distance {
                state.0.0 += 1;
                if !history.insert(state.0) {
                    println!("We've been here before, distance = {}", distance_to_origin(&state.0));
                }
            }
        },
        Direction::South => {
            for _i in 0..distance {
                state.0.1 -= 1;
                if !history.insert(state.0) {
                    println!("We've been here before, distance = {}", distance_to_origin(&state.0));
                }
            }
        },
        Direction::West => {
            for _i in 0..distance {
                state.0.0 -= 1;
                if !history.insert(state.0) {
                    println!("We've been here before, distance = {}", distance_to_origin(&state.0));
                }
            }
        }
    }
}

pub fn distance_to_origin(position: &Position) -> usize {
    (position.0).abs_diff(0) + (position.1).abs_diff(0)
}