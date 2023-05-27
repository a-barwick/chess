use super::{
    game::{Color, Game},
    player::Player,
};

pub fn start() {
    let player_one = Player::new("Player 1".to_string(), Color::White);
    let player_two = Player::new("Player 2".to_string(), Color::Black);
    let game = Game::new(&player_one, &player_two);
}
