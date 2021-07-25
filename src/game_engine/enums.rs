#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerState {
    Agent = 1,
    Empty = 0,
    Opponent = -1,
}

impl PlayerState {
    pub fn to_string(&self) -> String {
        match self {
            PlayerState::Agent => "1".to_string(),
            PlayerState::Empty => "0".to_string(),
            PlayerState::Opponent => "-1".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardState {
    Agent,
    Opponent,
    Undecided,
    Tie,
}

impl BoardState {
    pub fn from_player_state(plater_state: PlayerState) -> BoardState {
        match plater_state {
            PlayerState::Agent => BoardState::Agent,
            PlayerState::Empty => BoardState::Undecided,
            PlayerState::Opponent => BoardState::Opponent,
        }
    }
}
