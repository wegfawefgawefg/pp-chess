use crate::board::Game;
use crate::notation::{board_to_string, move_to_string, parse_move};
use crate::types::GameStatus;
use std::io::{self, Write};

pub fn run() {
    let mut game = Game::new();
    println!("pp-chess");
    println!("enter moves in coordinate form like e2e4, or type help / moves / quit");

    loop {
        println!();
        print!("{}", board_to_string(&game));
        print_status(&game);

        match game.status() {
            GameStatus::InProgress => {}
            _ => break,
        }

        print!("{} to move > ", game.turn.name());
        let _ = io::stdout().flush();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("input error");
            break;
        }
        let cmd = line.trim();
        if cmd.is_empty() {
            continue;
        }
        match cmd {
            "quit" | "exit" => break,
            "help" => {
                println!("commands:");
                println!("  e2e4     make a move");
                println!("  moves    list legal moves");
                println!("  help     show help");
                println!("  quit     exit");
            }
            "moves" => {
                let mut legal: Vec<String> =
                    game.legal_moves().into_iter().map(move_to_string).collect();
                legal.sort();
                println!("{}", legal.join(" "));
            }
            _ => match parse_move(cmd) {
                Ok(mv) => {
                    if let Err(err) = game.make_move(mv) {
                        println!("{err}");
                    }
                }
                Err(err) => println!("{err}"),
            },
        }
    }
}

fn print_status(game: &Game) {
    match game.status() {
        GameStatus::InProgress => {
            if game.is_in_check(game.turn) {
                println!("{} is in check.", game.turn.name());
            }
        }
        GameStatus::Checkmate(loser) => {
            println!("checkmate. {} loses.", loser.name());
        }
        GameStatus::Stalemate => println!("stalemate."),
        GameStatus::DrawThreefold => println!("draw by repetition."),
        GameStatus::DrawFiftyMove => println!("draw by fifty-move rule."),
        GameStatus::DrawInsufficientMaterial => println!("draw by insufficient material."),
    }
}
