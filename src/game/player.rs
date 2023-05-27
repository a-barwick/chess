use super::game::Color;

pub struct Player {
    name: String,
    color: Color,
}

impl Player {
    pub fn new(name: String, color: Color) -> Self {
        Player { name, color }
    }
}
