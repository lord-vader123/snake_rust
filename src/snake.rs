pub struct Snake {
    pub rows: Vec<u16>,
    pub columns: Vec<u16>,
    direction: char,
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
        if self.rows.len() == 1 {
            match self.direction {
                'w' => {
                    self.rows.push(self.rows[0] - 1);
                    self.columns.push(self.columns[0]);
                }
                's' => {
                    self.rows.push(self.rows[0] + 1);
                    self.columns.push(self.columns[0]);
                }
                'a' => {
                    self.columns.push(self.columns[0] - 1);
                    self.rows.push(self.rows[0]);
                }
                'd' => {
                    self.columns.push(self.columns[0] + 1);
                    self.rows.push(self.rows[0]);
                }
                _ => return,
            }
        } else {
            let last_row = self.rows[self.rows.len() - 1];
            let last_column = self.columns[self.columns.len() - 1];
            self.rows.push(last_row);
            self.columns.push(last_column);
        }
    }
}
