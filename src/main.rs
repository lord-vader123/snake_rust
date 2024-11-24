use std::{sync::mpsc, thread::sleep, time::Duration};

mod apple;
mod snake;
mod utils;

fn main() {
    let window = utils::Window::new();
    let mut snake = snake::Snake::new(window.columns, window.rows);
    let apple = apple::Apple::new(window.columns, window.rows);

    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        utils::get_input(tx);
    });

    loop {
        utils::clear_terminal();
        window.draw_game(&snake, &apple);
        let input = rx.recv().expect("Failed to receive input");
        snake.change_snake_direction(input);
        snake.move_snake();
        sleep(Duration::from_millis(100));
        continue;
    }
}
