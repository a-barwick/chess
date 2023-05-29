use serde::{Deserialize, Serialize};

use super::{board::Coordinate, game::Color};

#[derive(Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Serialize, Deserialize)]
pub struct Piece {
    pub kind: PieceType,
    pub color: Color,
    pub position: Coordinate,
    pub is_alive: bool,
}

impl Piece {
    pub fn new(kind: PieceType, color: Color, position: Coordinate) -> Self {
        Piece {
            kind,
            color,
            position,
            is_alive: true,
        }
    }

    pub fn get_available_moves(&self) -> Option<Vec<Coordinate>> {
        // get_available_moves(&self.kind, &self.position)
        None
    }

    pub fn is_players(&self, color: Color) -> bool {
        false
    }
}
