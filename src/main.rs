use ttt_rust::TicTacToe;

fn main() {
    let ttt = TicTacToe::new();

    print_board(ttt);
}

fn print_board(ttt: TicTacToe) {
    for (i, c) in ttt.board.into_iter().enumerate() {
        print!("{} ", c);

        if i % 3 == 0 {
            println!();
        }
    }
}
