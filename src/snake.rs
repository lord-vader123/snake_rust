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
        match direction {
            'w' | 's' | 'a' | 'd' => self.direction = direction,
            _ => return,
        }
    }

    pub fn move_snake(&mut self) {
        match self.direction {
            'w' => self.rows[0] -= 1,
            's' => self.rows[0] += 1,
            'a' => self.columns[0] -= 1,
            'd' => self.columns[0] += 1,
            _ => return,
        }

        for i in (1..self.rows.len()).rev() {
            self.rows[i] = self.rows[i - 1];
            self.columns[i] = self.columns[i - 1];
        }
    }

    pub fn grow_snake(&mut self) {
        let last_row = *self.rows.last().unwrap();
        let last_column = *self.columns.last().unwrap();

        self.rows.push(last_row);
        self.columns.push(last_column);
    }

    pub fn is_collision(&self, max_rows: u16, max_columns: u16) -> bool {
        let head_row = self.rows[0];
        let head_column = self.columns[0];

        if head_row <= 1
            || head_row >= max_rows - 1
            || head_column <= 1
            || head_column >= max_columns - 1
        {
            return true;
        }

        for i in 1..self.rows.len() {
            if head_row == self.rows[i] && head_column == self.columns[i] {
                return true;
            }
        }

        false
    }
}
