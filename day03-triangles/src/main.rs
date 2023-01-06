use std::fs;

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let content_copy = content.clone();

        let possible1: usize = content.lines()
            .map(|s| string_to_vec(s))
            .map(|v| possible(v))
            .sum();

        println!("First star: {}", possible1);

        // Second star

        let mut content_lines: Vec<&str> = content_copy.lines().collect();
        let mut possible2: usize = 0;

        while content_lines.len() != 0 {
            let block: Vec<Vec<usize>> = content_lines.drain(0..3)
                .map(|s| string_to_vec(s))
                .collect();

            for i in 0..3 {
                let temp_vec = vec![block[0][i], block[1][i], block[2][i]];
                possible2 += possible(temp_vec);
            }
        }

        println!("Second star: {}", possible2);
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

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
// fn transpose(v: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
//     let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

//     (0..3).map(|_| {
//         iters.iter_mut()
//             .map(|n| n.next().unwrap())
//             .collect::<Vec<usize>>()
//     })
//     .collect()
// }
