use std::{
    fs::{create_dir_all, read_dir, remove_file, OpenOptions},
    io::Write,
};

use uuid::Uuid;

use super::{
    game::{Color, Game},
    player::Player,
};

pub struct Session {
    id: Uuid,
    game: Game,
    is_save_on_exit: bool,
    save_dir: String,
}

impl Session {
    pub fn new(
        player_one_name: String,
        player_two_name: String,
        save_on_exit: bool,
        save_dir_path: String,
    ) -> Self {
        let player_one = Player::new(player_one_name, Color::White);
        let player_two = Player::new(player_two_name, Color::Black);
        let game = Game::new(player_one, player_two);
        Session {
            id: Uuid::new_v4(),
            game,
            is_save_on_exit: save_on_exit,
            save_dir: save_dir_path,
        }
    }

    /// TODO: Decomp to separate service
    pub fn save(&self) -> std::io::Result<()> {
        self.remove_files_in_save_dir()?;
        if !self.is_save_on_exit {
            return Ok(());
        }
        create_dir_all(self.save_dir.to_string())?;
        let game_state = serde_json::to_string(&self.game)?;
        let file_name = format!("data/saves/{}.json", self.id.to_string());

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_name)?;
        file.write_all(game_state.as_bytes())?;
        Ok(())
    }

    fn remove_files_in_save_dir(&self) -> Result<(), std::io::Error> {
        let dir = read_dir(self.save_dir.to_string())?;
        for entry in dir {
            let path = entry?.path();
            if path.is_file() {
                remove_file(path)?;
            }
        }
        Ok(())
    }
}
