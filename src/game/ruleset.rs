use super::board::Coordinate;
use super::piece::Kind;

pub fn get_available_moves(kind: &Kind, position: &Coordinate) -> Option<Vec<Coordinate>> {
    match kind {
        Kind::Pawn => Some(vec![Coordinate::new(position.x, position.y)]),
        Kind::Rook => Some(vec![Coordinate::new(position.x, position.y)]),
        Kind::Knight => Some(vec![Coordinate::new(position.x, position.y)]),
        Kind::Bishop => Some(vec![Coordinate::new(position.x, position.y)]),
        Kind::Queen => Some(vec![Coordinate::new(position.x, position.y)]),
        Kind::King => Some(vec![Coordinate::new(position.x, position.y)]),
    }
}
