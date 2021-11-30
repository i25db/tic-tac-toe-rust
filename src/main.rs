use ttt_rust::TicTacToe;
use std::io::{stdin, stdout, Write};
use std::string::String;

fn main() {
    let mut ttt = TicTacToe::new();
   
    println!("Welcome to a brand new game of Tic Tac Toe!");

    let mut turn = 0;
    let players = ['x', 'o'];

    while !ttt.is_game_over().0 {
        let current_player = players[turn % 2];

        print_board(&ttt);
        
        // let mut result = ttt.set(pos.0, pos.1, current_player);
        let mut pos = prompt_for_move(current_player);

        while let Some(msg) = ttt.set(pos.0, pos.1, current_player) {
            println!("{}", msg);

            pos = prompt_for_move(current_player);
        }
        
        let game_result = ttt.is_game_over();
        if game_result.0 {
            print_board(&ttt);
            match game_result.1 {
                '_' => {
                    println!("Oof! It was a draw.");
                    break;
                },
                c => {
                    println!("Congratulaions Player {}! You have won!!!", c.to_uppercase());
                    break;
                }
            }
        }
        
        turn += 1;
    }
}

fn prompt_for_move(player: char) -> (i32, i32) {
    print!("Player {} pick your move (x,y)>", player.to_uppercase());
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let split = input.trim_end().split(',').collect::<Vec<&str>>();

    if split.len() != 2 {
        println!("Don't forget a comma");

        prompt_for_move(player)
    } else {
        let x = split.get(0).unwrap().parse::<i32>();
        let y = split.get(1).unwrap().parse::<i32>();

        match x {
            Ok(x) => match y {
                Ok (y) => (x, y),
                Err(err) => {
                    println!("Please input two numbers for coordinates (x,y)");

                    prompt_for_move(player)
                }
            },
            Err(_) => {
                println!("Please input two numbers for coordinates (x,y)");

                prompt_for_move(player)

            }
        }
    }
}

fn print_board(ttt: &TicTacToe) {
    for (i, c) in ttt.into_iter().enumerate() {
        print!("{} ", c);

        if (i+1) % 3 == 0 {
            println!();
        }
    }
}
