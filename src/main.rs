mod apple;
mod snake;
mod utils;

fn main() {
    let window = utils::Window::new();
    let snake = snake::Snake::new(window.columns, window.rows);
    let apple = apple::Apple::new(window.columns, window.rows);

    window.draw_game(&snake, &apple);
    print!("Jabko: \nRow: {}\nColumn: {}\n", apple.row, apple.column);
    print!(
        "Okno: \nRow: {}\nColumn: {}\n\t\n",
        window.rows, window.columns
    );

    if apple.row > window.rows || apple.column > window.columns {
        print!("KOGOS POJEBALO");
    }
}
