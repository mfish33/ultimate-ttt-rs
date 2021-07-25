use super::enums::PlayerState;
use super::sub_board::SubBoard;
use super::types::{Coord, Move};

const SIZE: usize = 3;

pub struct LargeBoard {
    pub board: [[SubBoard; 3]; 3],
    previous_turns: Vec<Move>,
    pub state_board: SubBoard,
}

impl LargeBoard {
    pub fn new() -> Self {
        Self {
            board: [[SubBoard::new(); 3]; 3],
            previous_turns: Vec::new(),
            state_board: SubBoard::new(),
        }
    }

    pub fn make_move(&mut self, player: PlayerState, location: Move) {
        let mut sub_board = self.board[location.large_board.x][location.large_board.y];
        sub_board.make_move(player, location.sub_board);

        if sub_board.is_finished() {
            self.state_board.make_move(player, location.large_board);
        }

        self.previous_turns.push(location);
    }

    pub fn undo_move(&mut self) {
        let last_move = self
            .previous_turns
            .pop()
            .expect("Can not undo move in empty game");
        let mut sub_board = self.board[last_move.large_board.x][last_move.large_board.y];
        let was_finished = sub_board.is_finished();
        sub_board.undo_move(last_move.sub_board);

        if was_finished && !sub_board.is_finished() {
            self.state_board.undo_move(last_move.large_board);
        }
    }

    pub fn get_next_board(&self) -> Option<Coord> {
        self.previous_turns.last().map(|m| m.sub_board)
    }

    pub fn get_valid_boards(&self) -> Vec<Coord> {
        if let Some(next_board) = self.get_next_board() {
            return vec![next_board];
        }
        self.board
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, sub_board)| !sub_board.is_finished())
            .map(|(i, _)| Coord::new(i / SIZE, i % SIZE))
            .collect()
    }

    pub fn get_valid_moves(&self) -> Vec<Move> {
        self.get_valid_boards()
            .iter()
            .map(|large_board_cords| {
                self.board[large_board_cords.x][large_board_cords.y]
                    .get_valid_moves()
                    .into_iter()
                    .map(move |sub_board_cords| Move::new(*large_board_cords, sub_board_cords))
            })
            .flatten()
            .collect()
    }
}
