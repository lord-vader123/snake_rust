use rand::Rng;

pub struct Apple {
    pub row: u16,
    pub column: u16,
}

impl Apple {
    pub fn new(max_row: u16, max_column: u16) -> Self {
        let mut rng = rand::thread_rng();

        Apple {
            row: rng.gen_range(2..max_row - 1),
            column: rng.gen_range(2..max_column - 1),
        }
    }
}

impl Clone for Apple {
    fn clone(&self) -> Self {
        Apple {
            row: self.row,
            column: self.column,
        }
    }
}
