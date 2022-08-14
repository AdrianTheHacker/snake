pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub direction: char,
    pub position: usize, // 0 = Not part of snake
                     // 1 = Head of snake
                     // 2+ = body of snake
}

pub fn make_gameboard(width: usize, height: usize) -> Vec<Pixel> {
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

pub fn draw_gameboard(gameboard: &Vec<Pixel>, width: usize, height: usize, ) {
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

pub fn draw_grid_cords(width: usize, height: usize) {
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