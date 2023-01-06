use std::fs;
use std::collections::HashSet;
use day01_hq::*;

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let instructions: Vec<&str> = content.split(", ").collect();
        let mut state: (Position, Direction) = ((0, 0), Direction::North);
        let mut history: History = HashSet::new();
        
        history.insert(state.0);

        let mut instruction_iter = instructions.iter();

        while let Some(next_instruction) = instruction_iter.next() {
           new_position(&mut state, next_instruction.trim(), &mut history);
        }
        println!("Distance from origin: {}", distance_to_origin(&state.0));
    }
}
