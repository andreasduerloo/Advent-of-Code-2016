use std::fs;

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let copy_content = content.clone();

        let mut first_pos: Position = (1, 1);
        let mut first_code: usize = 0;

        let mut second_pos: Position = (2, 0);
        let mut second_code: usize = 0; // We'll work in hexadecimal

        content.lines().enumerate().for_each(|(i, s)| first_code += 10usize.pow((4 - i).try_into().unwrap()) * find_number(s, &mut first_pos, Star::First));

        println!("The first bathroom code is {}", first_code);

        copy_content.lines().enumerate().for_each(|(i, s)| second_code += 16usize.pow((4 - i).try_into().unwrap()) * find_number(s, &mut second_pos, Star::Second));

        println!("The second bathroom code is {:#x}", second_code);

    }
}

type Position = (usize, usize);

enum Star {
    First,
    Second,
}

fn find_number(instruction: &str, position: &mut Position, star: Star) -> usize {
    match star {
        Star::First => {
            let numpad: [[usize; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    
            instruction.chars().for_each(|c| apply_instruction(c, position, Star::First));

            numpad[position.0][position.1]
        },
        Star::Second => {
            let numpad: [[usize; 5]; 5] = [[0, 0, 1, 0, 0], [0, 2, 3, 4, 0], [5, 6, 7, 8, 9], [0, 0xA, 0xB, 0xC, 0], [0, 0, 0xD, 0, 0]];

            instruction.chars().for_each(|c| apply_instruction(c, position, Star::Second));

            numpad[position.0][position.1]
        }
    }
}

fn apply_instruction(instr: char, position: &mut Position, star: Star) {
    match instr {
        'U' => {
            match star {
                Star::First => {
                    if position.0 != 0 {
                        position.0 -= 1;
                    }
                },
                Star::Second => {
                    match position.0 {
                        0 => {
                            return
                        },
                        1 => {
                            if position.1 == 2 {
                                position.0 -= 1;
                            }
                        },
                        2 => {
                            if position.1 >= 1 && position.1 <= 3 {
                                position.0 -= 1;
                            }
                        },
                        _ => {
                            position.0 -= 1;
                        }
                    }
                }
            }
        },
        'R' => {
            match star {
                Star::First => {
                    if position.1 != 2 {
                        position.1 += 1;
                    }
                },
                Star::Second => {
                    match position.1 {
                        4 => {
                            return
                        },
                        3 => {
                            if position.0 == 2 {
                                position.1 += 1;
                            }
                        },
                        2 => {
                            if position.0 >= 1 && position.0 <= 3 {
                                position.1 += 1;
                            }
                        },
                        _ => {
                            position.1 += 1;
                        }
                    }
                }
            }
        },
        'D' => {
            match star {
                Star::First => {
                    if position.0 != 2 {
                        position.0 += 1;
                    }
                },
                Star::Second => {
                    match position.0 {
                        4 => {
                            return
                        },
                        3 => {
                            if position.1 == 2 {
                                position.0 += 1;
                            }
                        },
                        2 => {
                            if position.1 >= 1 && position.1 <= 3 {
                                position.0 += 1;
                            }
                        },
                        _ => {
                            position.0 += 1;
                        }
                    }
                }
            }
        },
        'L' => {
            match star {
                Star::First => {
                    if position.1 != 0 {
                        position.1 -= 1;
                    }
                },
                Star::Second => {
                    match position.1 {
                        0 => {
                            return
                        },
                        1 => {
                            if position.0 == 2 {
                                position.1 -= 1;
                            }
                        },
                        2 => {
                            if position.0 >= 1 && position.0 <= 3 {
                                position.1 -= 1;
                            }
                        },
                        _ => {
                            position.1 -= 1;
                        }
                    }
                }
            }      
        },
        _ => { return }
    }
}

