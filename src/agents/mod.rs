use super::game_engine::types::Move;
pub mod min_max;

pub trait Agent {
    fn new() -> Self;
    fn init(&mut self);
    fn make_move(&mut self) -> Move;
    fn make_first_move(&mut self) -> Move {
        self.make_move()
    }
    fn opponent_move(&mut self, location: Move);
}
