use std::fs;
use std::collections::HashSet;
use day01_hq::*;

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let instructions: Vec<&str> = content.split(", ").collect();
        let mut state: (Position, Direction, bool) = ((0, 0), Direction::North, false);
        let mut history: History = HashSet::new();
        
        history.insert(state.0);

        instructions.iter().for_each(|s| new_position(&mut state, s.trim(), &mut history));

        println!("Distance from origin: {}", distance_to_origin(&state.0));
    }
}
