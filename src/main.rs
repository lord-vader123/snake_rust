use crossterm::event::{self, Event, KeyCode};

use crossterm::terminal;

use std::{io, time::Duration};

mod snake;
mod utils;
mod apple;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    
    let window = utils::Window::new();
    let mut snake = snake::Snake::new(window.columns, window.rows);
    let mut apple = apple::Apple::new(window.columns, window.rows);

    loop {
        utils::clear_terminal();
        window.draw_game(&snake, &apple);

        if snake.is_collision(window.rows, window.columns) {
            println!("Game Over!");
            break;
        }

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) if matches!(c, 'w' | 's' | 'a' | 'd') => {
                        snake.change_snake_direction(c);
                    }
                    KeyCode::Char('q') => {
                        println!("Quit!");
                        break;
                    }
                    _ => {}
                }
            }
        }

        snake.move_snake();
        apple = window.eat_apple(&mut snake, &apple);
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
