use ttt_rust::TicTacToe;
use std::io::{stdin, BufRead};

fn main() {
    let ttt = TicTacToe::new();

    print_board(ttt);
}

fn prompt_for_move(player: char) -> (i8, i8) {
    print!("Play '{}' pick location (#,#)>", player);
    
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let split = input.split(',').collect::<Vec<&str>>();

    if split.len() != 2 {
        println!("Please input two numbers for coordinates (#,#)");

        prompt_for_move(player)
    } else {
        let x = split[0].parse::<i8>();
        let y = split[1].parse::<i8>();

        match x {
            Ok(x) => match y {
                Ok (y) => (x, y),
                Err(e) => {
                    println!("Please input two numbers for coordinates (#,#)");

                    prompt_for_move(player)
                }
            },
            Err(e) => {
                println!("Please input two numbers for coordinates (#,#)");

                prompt_for_move(player)

            }
        }
    }
}

fn print_board(ttt: TicTacToe) {
    for (i, c) in ttt.board.into_iter().enumerate() {
        print!("{} ", c);

        if i % 3 == 0 {
            println!();
        }
    }
}
