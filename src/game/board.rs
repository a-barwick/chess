use serde::{Deserialize, Serialize};

use super::{game::Color, piece::Piece};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Square {
    pub position: Coordinate,
    pub piece: Option<Piece>,
}

impl Square {
    pub fn new(position: Coordinate, piece: Option<Piece>) -> Self {
        Square { position, piece }
    }

    pub fn is_occupied(&self) -> bool {
        self.piece.is_some()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    squares: Vec<Vec<Square>>,
}

impl Board {
    pub fn new(squares: Vec<Vec<Square>>) -> Self {
        Board { squares }
    }

    pub fn get_piece_at(&self, position: Coordinate) -> Option<&Piece> {
        if self.is_within_bounds(position) && self.contains_piece(position) {
            return self.squares[position.x][position.y].piece.as_ref();
        }
        None
    }

    pub fn is_within_bounds(&self, position: Coordinate) -> bool {
        position.x >= 0 && position.x <= 7 && position.y >= 0 && position.y <= 7
    }

    pub fn contains_piece(&self, position: Coordinate) -> bool {
        self.squares[position.x][position.y].is_occupied()
    }
}

pub fn get_piece_for_default_position(position: Coordinate) -> Option<Piece> {
    None
}
