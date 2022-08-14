use std::io::stdin;
use crate::gameboard::Pixel;

pub fn add_snake(gameboard: &mut Vec<Pixel>) {
    gameboard[0].direction = 'R'; // Sets the top left pixel's direction to rigth.
    gameboard[0].position = 1; // Makes the top left pixel of the screen the head of the snake.
}

pub fn move_snake(gameboard: &mut Vec<Pixel>, direction: char, width: usize, height: usize) {
    for index in 0..gameboard.len() - 1 {
        if gameboard[index].position == 1 {
            // move in direction

            let mut next_spot = index;

            match direction {
                'U' => {
                    if index > width - 1 {  // Top pixels
                        next_spot = index - width;
                    }
                },

                'D' => {
                    if index + width <= gameboard.len() - width {  // Bottom pixels
                        next_spot = index + width;
                    }
                },

                'R' => {
                    if (index + 1) % width != 0 {
                        next_spot = index + 1;
                    }
                },

                'L' => {
                    if index % width != 0 {
                        next_spot = index - 1;
                    }
                }

                _ => println!("This is why we can't have nice things"),
            }

            gameboard[next_spot].direction = direction;
            gameboard[next_spot].position = gameboard[index].position;
            if next_spot != index {
                gameboard[index].direction = 'N';
                gameboard[index].position = 0;
            }
            
            break;
        }
    }
}

pub fn get_next_move() -> char { 
    let mut next_move = String::new();
    stdin().read_line(&mut next_move)
    	.ok()
        .expect("Failed to read line");

    next_move.chars().next().unwrap()
}
