#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub large_board: Coord,
    pub sub_board: Coord,
}

impl Move {
    pub fn new(large_board: Coord, sub_board: Coord) -> Self {
        Self {
            large_board,
            sub_board,
        }
    }
}
