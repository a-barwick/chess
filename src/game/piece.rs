use super::{board::Coordinate, game::Color, ruleset::get_available_moves};

pub enum Kind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Piece {
    kind: Kind,
    color: Color,
    position: Coordinate,
    is_alive: bool,
}

impl Piece {
    pub fn new(kind: Kind, color: Color, position: Coordinate) -> Self {
        Piece {
            kind,
            color,
            position,
            is_alive: true,
        }
    }

    pub fn get_available_moves(&self) -> Option<Vec<Coordinate>> {
        get_available_moves(&self.kind, &self.position)
    }

    pub fn is_players(&self, color: Color) -> bool {
        false
    }
}
