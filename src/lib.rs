#[derive(Debug)]
pub struct TicTacToeBoard {
    board: [char; 9]
}

impl TicTacToeBoard {
    
    // Create a new, empty tic tac toe board
    pub fn new() {
        TicTacToeBoard {
            board: [' '; 9],
        };
    }

    fn pos(x: u8, y: u8) -> usize {
        ((x-1)*(y-1)) as usize
    }

    fn at(&self, x: u8, y: u8) -> Result<char, ()> {
        Ok(self.board[TicTacToeBoard::pos(x,y) as usize])
    }

    pub fn set(&mut self, x: u8, y: u8, piece: char) -> Result<(), String> {
        match self.at(x,y)  {
            Ok(c) => {
                if c != ' ' {
                    Err(String::from("Location is not empty"))
                } else {
                    self.board[TicTacToeBoard::pos(x,y)] = piece;
                    Ok(())
                }
            },
            _ => Err(String::from("Unknown error"))
        }
    }
}
