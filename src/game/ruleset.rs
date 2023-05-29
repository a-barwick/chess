use super::board::{Board, Coordinate};
use super::game::Color;
use super::piece::PieceType;

trait Ruleset {
    fn default_position(color: Color) -> Vec<Coordinate>;
    fn available_moves(
        color: Color,
        position: Coordinate,
        board: &Board,
    ) -> Option<Vec<Coordinate>>;
}

struct RuleEngine;

impl RuleEngine {}

struct PawnRuleset;

impl Ruleset for PawnRuleset {
    fn default_position(color: Color) -> Vec<Coordinate> {
        let mut coords = Vec::with_capacity(8);
        for i in 0..8 {
            let mut y = if color == Color::White { 1 } else { 6 };
            coords.push(Coordinate::new(i, y));
        }
        coords
    }

    fn available_moves(
        color: Color,
        position: Coordinate,
        board: &Board,
    ) -> Option<Vec<Coordinate>> {
        let mut moves = Vec::with_capacity(3);
        if color == Color::White {
            let f_move = {
                let mut m = position.clone();
                m.y + 1;
                m
            };
            if board.is_within_bounds(f_move) && !board.contains_piece(f_move) {
                moves.push(f_move);
            }
            let l_move = {
                let mut m = position.clone();
                m.y += 1;
                m.x -= 1;
                m
            };
            if let Some(piece) = board.get_piece_at(l_move) {
                if board.is_within_bounds(l_move)
                    && board.contains_piece(l_move)
                    && piece.color != color
                {
                    moves.push(l_move);
                }
            }
            let r_move = {
                let mut m = position.clone();
                m.y += 1;
                m.x -= 1;
                m
            };
            if let Some(piece) = board.get_piece_at(r_move) {
                if board.is_within_bounds(r_move)
                    && board.contains_piece(r_move)
                    && piece.color != color
                {
                    moves.push(r_move);
                }
            }
            return Some(moves);
        } else {
        }
        None
    }
}
