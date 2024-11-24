use std::io::Write;
use std::sync::mpsc;
use std::{io, thread};

use crate::apple::Apple;
use crate::snake::Snake;

use crossterm::terminal;

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
        Window { rows, columns }
    }

    pub fn draw_game(&self, snake: &Snake, apple: &Apple) {
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
            println!();
        }
    }

    pub fn eat_apple(&self, snake: &mut Snake, apple: &Apple) -> Apple {
        if apple.row == snake.rows[0] && apple.column == snake.columns[0] {
            snake.grow_snake();
            return Apple::spawn_apple(self.rows, self.columns);
        }
        apple.clone()
    }
}

pub fn get_input(tx: mpsc::Sender<char>) {
    std::thread::spawn(move || {
        let mut input = String::new();
        while let Ok(_) = io::stdin().read_line(&mut input) {
            let first_char = input.trim().chars().next().unwrap_or('\0');
            tx.send(first_char).expect("Failed to send message");
            input.clear();
        }
    });
}

pub fn clear_terminal() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();
}
