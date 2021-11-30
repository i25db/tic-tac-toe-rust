pub struct TicTacToe {
    pub board : [char; 9]
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            board: [' '; 9]
        }
    }

    pub fn at(&self, x: i8, y: i8) -> Option<char> {
        // Bounds check
        if x > 3 || x < 1 || y > 3 || y < 1 {
            None
        } else {
            Some(self.board[(x*y-1) as usize])
        }
    }

    // Returns None if no errors otherwise and error string
    pub fn set(&mut self, x: i8, y: i8, c: char) -> Option<String> {
        match self.at(x, y) {
            None => Some(String::from("Position out of bounds")),
            Some(ch) => {
                if ch != ' ' {
                    Some(String::from("Position not empty"))
                } else {
                    None
                }
            }
        }
    }
}
