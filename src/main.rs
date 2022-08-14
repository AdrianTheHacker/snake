use std::io::stdin;

fn main() {
    println!("Snake Game!");

    const WIDTH: usize = 9;
    const HEIGHT: usize = 5;

    let mut gameboard = make_gameboard(WIDTH, HEIGHT);

    draw_gameboard(&gameboard, WIDTH, HEIGHT);
    add_snake(&mut gameboard);

    loop {
        draw_gameboard(&gameboard, WIDTH, HEIGHT);
        move_snake(&mut gameboard, get_next_move(), WIDTH, HEIGHT);
    }
}

fn make_gameboard(width: usize, height: usize) -> Vec<Pixel> {
    let mut gameboard = Vec::new();

    for x in 0..width {
        for y in 0..height {
            gameboard.push(
                Pixel {
                    x: x,
                    y: y,
                    direction: 'N',
                    position: 0 // Not a part of the snake+
                }
            );
        }
    }

    gameboard
}

fn draw_gameboard(gameboard: &Vec<Pixel>, width: usize, height: usize, ) {
    for y in 0..height {
        for x in 0..width {
            match gameboard[x + (y * width)].direction {
                'N' => print!("{}", "O"),
                'U' => print!("{}", "^"),
                'D' => print!("{}", "v"),
                'L' => print!("{}", "<"),
                'R' => print!("{}", ">"),
                _ => println!("This is why we can't have nice things"),
            }
        }
        println!("");
    }
    println!("");
}

fn draw_grid_cords(width: usize, height: usize) {
    for pixel_x in 0..width {
        for pixel_y in 0..height {
            if pixel_y == height {
                println!("[{}, {}]", pixel_x, pixel_y);
                println!("new line");
            }

            else {
                print!("[{}, {}]", pixel_x, pixel_y);
            }
        }
    }
}

fn add_snake(gameboard: &mut Vec<Pixel>) {
    gameboard[0].direction = 'R'; // Sets the top left pixel's direction to rigth.
    gameboard[0].position = 1; // Makes the top left pixel of the screen the head of the snake.
}

fn move_snake(gameboard: &mut Vec<Pixel>, direction: char, width: usize, height: usize) {
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

fn get_next_move() -> char { 
    let mut next_move = String::new();
    stdin().read_line(&mut next_move)
    	.ok()
        .expect("Failed to read line");

    next_move.chars().next().unwrap()
}

struct Pixel {
    x: usize,
    y: usize,
    direction: char,
    position: usize, // 0 = Not part of snake
                     // 1 = Head of snake
                     // 2+ = body of snake
}
