use std::io::{self, BufRead};
pub mod game_engine;
use game_engine::types::{Coord, Move};
pub mod agents;
use agents::Agent;

fn main() {
    let mut player = agents::min_max::MinMax::new();
    let mut line = String::new();
    let stdin = io::stdin();
    loop {
        line.clear();
        stdin.lock().read_line(&mut line).unwrap();
        let trimmed = line.trim();
        let parts: Vec<&str> = trimmed.split(" ").collect();
        match parts[0] {
            "init" => player.init(),
            "move" => {
                let location = player.make_first_move();
                write_move(location)
            }
            "opponent" => {
                let opponent_move = parse_move(parts[1]);
                player.opponent_move(opponent_move);
                let location = player.make_move();
                write_move(location)
            },
            "timeout" => {},
            "game" => {},
            "match" => {}
            _ => println!("Bad input"),
        }
    }
}

fn write_move(location: Move) {
    println!(
        "send:{},{};{},{}",
        location.large_board.x, location.large_board.y, location.sub_board.x, location.sub_board.y
    )
}

fn parse_move(input: &str) -> Move {
    let boards: Vec<&str> = input.split(";").collect();
    let large_board: Vec<&str> = boards[0].split(",").collect();
    let sub_board: Vec<&str> = boards[1].split(",").collect();
    Move::new(
        Coord::new(
            large_board[0].parse().unwrap(),
            large_board[1].parse().unwrap(),
        ),
        Coord::new(sub_board[0].parse().unwrap(), sub_board[1].parse().unwrap()),
    )
}
