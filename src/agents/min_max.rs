use super::Agent;
use crate::game_engine::{
    enums::PlayerState,
    large_board::LargeBoard,
    types::{Coord, Move},
};
use std::collections::HashMap;
use std::fs;
use serde_json;

pub struct MinMax {
    game: LargeBoard,
    sub_board_scores:HashMap<String, isize>,
    large_board_scores:HashMap<String, isize>
}

impl Agent for MinMax {
    fn new() -> Self {
        let large_board_import = fs::read_to_string("./large_board_map.json").unwrap();
        let large_board_scores:HashMap<String, isize> = serde_json::from_str(&large_board_import).expect("Could not parse large board scores");
        let sub_board_scores = fs::read_to_string("./small_board_map.json").unwrap();
        let sub_board_scores:HashMap<String, isize> = serde_json::from_str(&sub_board_scores).expect("Could not parse small board scores");
        Self {
            game: LargeBoard::new(),
            sub_board_scores,
            large_board_scores
        }
    }

    fn init(&mut self) {
        self.game = LargeBoard::new()
    }

    fn opponent_move(&mut self, location: Move) {
        self.game.make_move(PlayerState::Opponent, location)
    }

    fn make_move(&mut self) -> Move {
        self.minimax(0, true, isize::MIN, isize::MAX).1.unwrap()
    }

    fn make_first_move(&mut self) -> Move {
        let location = Move::new(Coord::new(1, 1), Coord::new(1, 1));
        self.game.make_move(PlayerState::Agent, location);
        location
    }
}

impl MinMax {
    fn get_score(&self) -> isize {
        let mut score = 0;
        for sub_board in self.game.board.iter().flatten() {
            let state = sub_board.to_string();
            score += self.sub_board_scores.get(&state).expect("sub board value does not exist");
        }
        let large_board_state = self.game.state_board.to_string();
        score += self.large_board_scores.get(&large_board_state).expect("large board value does not exist");
        score
    }

    fn minimax(&mut self, depth:usize, is_agent:bool, mut alpha: isize, mut beta: isize) -> (isize, Option<Move>) {
        let moves = self.game.get_valid_moves();

        if moves.len() == 0 || depth == 4 {
            let score = self.get_score();
            return (score, None);
        }

        let mut best_move = None;
        for location in moves {
            if is_agent {
                self.game.make_move(PlayerState::Agent, location);
            } else {
                self.game.make_move(PlayerState::Opponent, location);
            }
            let score = self.minimax(depth + 1, !is_agent, alpha, beta).0;
            if is_agent && score > alpha {
                alpha = score as isize;
                best_move = Some(location)
            } else if !is_agent && score < beta {
                beta = score;
                best_move = Some(location);
            }
            self.game.undo_move();
            if alpha > beta {
                break;
            }
        }
        let best_score = if is_agent {alpha} else {beta};
        (best_score, best_move)
    }
}
