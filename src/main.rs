pub mod gameboard;
pub mod snake;

fn main() {
    println!("Snake Game!");

    const WIDTH: usize = 9;
    const HEIGHT: usize = 5;

    let mut gameboard = gameboard::make_gameboard(WIDTH, HEIGHT);

    gameboard::draw_gameboard(&gameboard, WIDTH, HEIGHT);
    snake::add_snake(&mut gameboard);

    loop {
        gameboard::draw_gameboard(&gameboard, WIDTH, HEIGHT);
        snake::move_snake(&mut gameboard, snake::get_next_move(), WIDTH, HEIGHT);
    }
}
