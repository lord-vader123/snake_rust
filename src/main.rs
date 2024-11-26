use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode};

use crossterm::{execute, terminal};

use std::io::stdout;
use std::{io, time::Duration};

mod apple;
mod snake;
mod utils;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), Hide).unwrap();

    let window = utils::Window::new();
    let mut snake = snake::Snake::new(window.columns, window.rows);
    let mut apple = apple::Apple::new(window.columns, window.rows);

    loop {
        window.draw_game(&snake, &apple);

        if snake.is_collision(window.rows, window.columns) {
            println!("Game Over!");
            break;
        }

        if event::poll(Duration::from_millis(800))? {
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

        window.eat_apple(&mut snake, &mut apple);
        if snake.is_win(window.rows, window.columns) {
            print!("You won!");
            break;
        }
        snake.move_snake();
        print!("Wąż: {}, {}", snake.rows[0], snake.columns[0]);
        apple = window.eat_apple(&mut snake, &apple);
    }

    execute!(stdout(), Show).unwrap();
    terminal::disable_raw_mode()?;
    Ok(())
}
