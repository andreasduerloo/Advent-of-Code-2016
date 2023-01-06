use std::fs;

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        // let mut possible: usize = 0;

        let possible = content.lines()
            .map(|s| string_to_vec(s))
            .map(|v| possible(v))
            .reduce(|acc, val| acc + val)
            .unwrap();

        println!("First star: {}", possible);
    }
}

fn string_to_vec(line: &str) -> Vec<usize> {
    line.split_whitespace().map(|s| usize::from_str_radix(s, 10).unwrap()).collect()
}

fn possible(mut sides: Vec<usize>) -> usize {
    sides.sort();

    let two_sides: usize = sides.drain(0..2).sum();

    if two_sides > sides[0] {
        return 1;
    } else {
        0
    }
}
