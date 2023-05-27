use super::{
    board::{get_piece_for_default_position, Board, Coordinate, Square},
    player::Player,
};

pub enum Color {
    White,
    Black,
}

pub struct Game<'a> {
    state: String,
    player_one: &'a Player,
    player_two: &'a Player,
    cur_player: &'a Player,
    turn: usize,
    board: Board,
}

impl<'a> Game<'a> {
    pub fn new(player_one: &'a Player, player_two: &'a Player) -> Self {
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
            state: String::new(),
            player_one,
            player_two,
            cur_player: player_one,
            turn: 1,
            board,
        }
    }
}
