use std::array::IntoIter;

pub struct TicTacToe {
    board : [char; 9]
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            board: ['_'; 9]
        }
    }
    // transforms a position starting at origin 1,1 into an index
    fn pos(x: i32, y: i32) -> usize {
        (3 * y + x - 4) as usize
    }

    pub fn into_iter(&self) -> IntoIter<char, 9_usize> {
        self.board.into_iter()
    } 

    pub fn at(&self, x: i32, y: i32) -> Option<char> {
        // Bounds check
        if x > 3 || x < 1 || y > 3 || y < 1 {
            None
        } else {
            Some(self.board[TicTacToe::pos(x, y)])
        }
    }

    // Returns None if no errors otherwise and error string
    pub fn set(&mut self, x: i32, y: i32, c: char) -> Option<String> {
        match self.at(x, y) {
            None => Some(String::from("Position out of bounds")),
            Some(ch) => {
                if ch != '_' {
                    Some(String::from("Position not empty"))
                } else {
                    self.board[TicTacToe::pos(x, y)] = c;
                    None
                }
            }
        }
    }

    // returns true or false if the game is over or not
    // and if the game is over returns 'x' or 'y' for the winner
    // or '_' for a draw
    pub fn is_game_over(&self) -> (bool, char) {
        // Each element is a Row, Column or Diagonal and represents
        // how many x's and how many o's are in it
        // 0-2: rows
        // 3-5: columns
        // 6: 1,1 diagonal
        // 7: 3,1 diagonal
        let mut rcd = [(0, 0); 8];
        let mut empty_positions = 9;

        for (i, c) in self.board.into_iter().enumerate() {
            let x = i % 3;
            let y = (i / 3) % 3;

            match c {
                'x' => {
                    empty_positions -= 1;

                    // increment x count for row and column
                    rcd[x].0 += 1;
                    rcd[y+3].0 += 1;

                    // i is on diagonal 1,1 when i % 4 == 0
                    if i % 4 == 0 {
                        rcd[6].0 += 1;
                        println!("{}: {},{} Diagonal Count: {}", i, x, y, rcd[6].0);
                    }
                    // position is on diagonal 3,1 when x + y == 4
                    if x + y + 2 == 4 {
                        rcd[7].0 += 1;
                    }

                    if rcd[x].0 == 3 || rcd[y+3].0 == 3 || rcd[6].0 == 3 || rcd[7].0 == 3 {
                        return (true, 'x');
                    }
                },
                'o' => {
                    empty_positions -= 1;

                    // increment y count for row and column
                    rcd[x].1 += 1;
                    rcd[y+3].1 += 1;

                    if i % 4 == 0 {
                        rcd[6].1 += 1;
                    }

                    if x + y == 4 {
                        rcd[7].1 += 1;
                    }

                    if rcd[x].1 == 3 || rcd[y+3].1 == 3 || rcd[6].1 == 3 || rcd[7].1 == 3 {
                        return (true, 'o');
                    }
                },
                _ => ()
            };
        }

        (empty_positions == 0, '_')
    }
}
