use super::enums::{BoardState, PlayerState};
use super::types::Coord;

const SIZE: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubBoard {
    board: [[PlayerState; 3]; 3],
    winner: Option<BoardState>,
    move_count: usize,
    move_limit: usize,
}

impl SubBoard {
    pub fn new() -> Self {
        Self {
            board: [[PlayerState::Empty; 3]; 3],
            winner: None,
            move_count: 0,
            move_limit: SIZE.pow(2),
        }
    }

    pub fn make_move(&mut self, player: PlayerState, location: Coord) {
        if self.is_finished() {
            return;
        }

        self.board[location.x][location.y] = player;
        self.move_count += 1;

        // Check if the board has been won
        self.check_row(location.x);

        if !self.is_finished() {
            self.check_column(location.y);
        }

        if !self.is_finished() {
            self.check_ltr_diagonal();
        }

        if !self.is_finished() {
            self.check_rtl_diagonal();
        }

        // check for a tie
        if !self.is_finished() && self.move_count == self.move_limit {
            self.winner = Some(BoardState::Tie);
        }
    }

    pub fn is_finished(&self) -> bool {
        self.winner.is_some()
    }

    pub fn to_string(&self) -> String {
        self.board
            .iter()
            .flatten()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn undo_move(&mut self, location: Coord) {
        self.board[location.x][location.y] = PlayerState::Empty;
        self.move_count -= 1;
        self.winner = None;
    }

    pub fn get_valid_moves(&self) -> Vec<Coord> {
        let mut out = Vec::new();
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] == PlayerState::Empty {
                    out.push(Coord::new(i, j));
                }
            }
        }
        out
    }

    fn check_row(&mut self, row: usize) {
        let player = self.board[row][0];
        if player == PlayerState::Empty {
            return;
        }
        for i in 1..SIZE {
            if player != self.board[row][i] {
                return;
            }
        }
        self.winner = Some(BoardState::from_player_state(player))
    }

    /**
     * Check if a given column has been won
     * @param col Column index
     */
    fn check_column(&mut self, col: usize) {
        let player = self.board[0][col];
        if player == PlayerState::Empty {
            return;
        }
        for i in 1..SIZE {
            if player != self.board[i][col] {
                return;
            }
        }
        self.winner = Some(BoardState::from_player_state(player))
    }

    fn check_ltr_diagonal(&mut self) {
        let player = self.board[0][0];
        if player == PlayerState::Empty {
            return;
        }
        for i in 1..SIZE {
            if player != self.board[i][i] {
                return;
            }
        }
        self.winner = Some(BoardState::from_player_state(player))
    }

    /**
     * Check if the right to left diagonal has been won
     */
    fn check_rtl_diagonal(&mut self) {
        let player = self.board[0][SIZE - 1];
        if player == PlayerState::Empty {
            return;
        }
        for i in SIZE - 1..=0 {
            if player != self.board[SIZE - 1 - i][i] {
                return;
            }
        }
        self.winner = Some(BoardState::from_player_state(player))
    }
}
