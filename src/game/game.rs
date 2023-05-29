use serde::{Deserialize, Serialize};

use super::{
    board::{get_piece_for_default_position, Board, Coordinate, Square},
    player::Player,
};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    player_one: Player,
    player_two: Player,
    cur_player: Option<Player>,
    turn: usize,
    board: Board,
}

impl Game {
    pub fn new(player_one: Player, player_two: Player) -> Self {
        let mut squares = vec![Vec::with_capacity(8)];
        for i in 0..8 {
            let mut row = Vec::new();
            for j in 0..8 {
                let position = Coordinate::new(i, j);
                let piece = get_piece_for_default_position(position);
                row.push(Square::new(position, piece));
            }
            squares.push(row);
        }
        let board = Board::new(squares);
        Game {
            player_one,
            player_two,
            cur_player: None,
            turn: 1,
            board,
        }
    }
}
