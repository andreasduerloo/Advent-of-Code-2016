use std::fs;
use std::collections::HashSet;
use day01_hq::*;

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let mut state: (Position, Direction, bool) = ((0, 0), Direction::North, false);
        let mut history: History = HashSet::from([state.0]);

        content.split(", ").for_each(|s| new_position(&mut state, s.trim(), &mut history));

        println!("Distance from origin: {}", distance_to_origin(&state.0));
    }
}
