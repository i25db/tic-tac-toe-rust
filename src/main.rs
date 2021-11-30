use ttt_rust::TicTacToe;
use std::io::stdin;

fn main() {
    let mut ttt = TicTacToe::new();
    
    ttt.set(1, 1, 'o');
    ttt.set(2, 1, 'x');
    ttt.set(3, 1, 'o');
    print_board(&ttt);

    println!("Winner is: {}", ttt.is_game_over().unwrap());
}

fn prompt_for_move(player: char) -> (i8, i8) {
    print!("Player '{}' pick location (#,#)>", player);
    
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
                Err(_) => {
                    println!("Please input two numbers for coordinates (#,#)");

                    prompt_for_move(player)
                }
            },
            Err(_) => {
                println!("Please input two numbers for coordinates (#,#)");

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
