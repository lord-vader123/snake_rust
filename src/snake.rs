pub struct Snake {
    pub rows: Vec<u16>,
    pub columns: Vec<u16>,
    pub direction: char,
}

impl Snake {
    pub fn new(window_columns: u16, window_rows: u16) -> Self {
        let half_columns = window_columns / 2;
        let half_rows = window_rows / 2;

        Snake {
            rows: vec![half_rows],
            columns: vec![half_columns],
            direction: 'w',
        }
    }

    pub fn change_snake_direction(&mut self, direction: char) {
        let opposite = match self.direction {
            'w' => 's',
            's' => 'w',
            'a' => 'd',
            'd' => 'a',
            _ => return,
        };
        match direction {
            'w' | 's' | 'a' | 'd' => {
                if direction != opposite {
                    self.direction = direction;
                }
            }
            _ => return,
        }
    }

    pub fn move_snake(&mut self) {
        for i in (1..self.rows.len()).rev() {
            self.rows[i] = self.rows[i - 1];
            self.columns[i] = self.columns[i - 1];
        }

        match self.direction {
            'w' => {
                if self.rows[0] > 1 {
                    self.rows[0] -= 1;
                }
            }
            's' => {
                if self.rows[0] < u16::MAX {
                    self.rows[0] += 1;
                }
            }
            'a' => {
                if self.columns[0] > 0 {
                    self.columns[0] -= 1;
                }
            }
            'd' => {
                if self.columns[0] < u16::MAX {
                    self.columns[0] += 1;
                }
            }
            _ => {}
        }
    }

    pub fn grow_snake(&mut self) {
        let last_row = *self.rows.last().unwrap();
        let last_column = *self.columns.last().unwrap();

        self.rows.push(last_row);
        self.columns.push(last_column);
    }

    pub fn is_collision(&self, max_rows: u16, max_columns: u16) -> bool {
        return Snake::is_collision_self(self)
            || Snake::is_collision_wall(self, max_rows, max_columns);
    }

    fn is_collision_self(&self) -> bool {
        for (&snake_row, &snake_column) in self.rows.iter().zip(self.columns.iter()).skip(2) {
            if self.rows[0] == snake_row && self.columns[0] == snake_column {
                print!("You hit yourself! ");
                return true;
            }
        }
        false
    }

    fn is_collision_wall(&self, max_rows: u16, max_columns: u16) -> bool {
        if self.rows[0] <= 1
            || self.rows[0] >= max_rows - 1
            || self.columns[0] == 0
            || self.columns[0] >= max_columns - 1
        {
            print!("You hit the wall! ");
            return true;
        }
        false
    }

    pub fn is_win(&self, max_rows: u16, max_columns: u16) -> bool {
        let total_fields = (max_rows - 2) * (max_columns - 2);
        let snake_size = self.rows.len() as u16;

        snake_size >= total_fields
    }
}
