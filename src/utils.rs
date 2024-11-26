use crossterm::terminal;

use std::{io::{self, Write}, clone::Clone};

use crate::apple::Apple;
use crate::snake::Snake;

pub struct Window {
    pub rows: u16,
    pub columns: u16,
}

impl Window {
    pub fn new() -> Self {
        let (columns, rows) = match terminal::size() {
            Ok(size) => size,
            Err(_) => {
                eprintln!("Error during getting window size");
                std::process::exit(1);
            }
        };
        Window { columns, rows }
    }

    pub fn draw_game(&self, snake: &Snake, apple: &Apple) {
        print!("\x1B[H");

        for row in 0..self.rows {
            for column in 0..self.columns {
                if row == 1 || row == self.rows - 1 {
                    print!("-");
                } else if column == 0 || column == self.columns - 1 {
                    print!("|");
                } else if row == snake.rows[0] && column == snake.columns[0] {
                    print!("O");
                } else if row == apple.row && column == apple.column {
                    print!("A");
                } else {
                    let mut body = false;
                    for i in 0..snake.rows.len() {
                        if row == snake.rows[i] && column == snake.columns[i] {
                            print!("o");
                            body = true;
                            break;
                        }
                    }

                    if !body {
                        print!(" ");
                    }
                }
            }
        }
        io::stdout().flush().unwrap();
    }

    pub fn eat_apple(&self, snake: &mut Snake, apple: &Apple) -> Apple {
        if apple.row == snake.rows[0] && apple.column == snake.columns[0] {
            snake.grow_snake();
            return Apple::spawn_apple(self.rows, self.columns);
        }
        apple.clone()
    }
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}
