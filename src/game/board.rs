use super::piece::{Kind, Piece};

#[derive(Copy, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
}

pub struct Square {
    position: Coordinate,
    is_occupied: bool,
    piece: Option<Piece>,
}

impl Square {
    pub fn new(position: Coordinate, piece: Option<Piece>) -> Self {
        Square {
            position,
            is_occupied: piece.is_some(),
            piece,
        }
    }
}

pub struct Board {
    squares: Vec<Vec<Square>>,
}

impl Board {
    pub fn new(squares: Vec<Vec<Square>>) -> Self {
        Board { squares }
    }
}

pub fn get_piece_for_default_position(position: Coordinate) -> Option<Piece> {
    None
}
