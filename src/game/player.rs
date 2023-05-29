use serde::{Deserialize, Serialize};

use super::game::Color;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    name: String,
    color: Color,
}

impl Player {
    pub fn new(name: String, color: Color) -> Self {
        Player { name, color }
    }
}
