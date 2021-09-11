//! Module for all pod structures

use super::{config};

/// Struct storing the games message stream.
pub struct GameLog {
    /// [Vec] containing the message
    /// stream of the game.
    pub messages: Vec<String>,
}

impl GameLog {
    /// Creates a new [GameLog] and prefills
    /// it with the games name, version and an
    /// introductory message.
    pub fn new() -> Self {
        GameLog {
            messages: vec![
                format!("{} {}", config::GAME_NAME, config::GAME_VERSION),
                "You entered the dungeon...".to_string(),
            ],
        }
    }

    /// Creates a new [GameLog] with an empty
    /// message stream.
    pub fn new_empty() -> Self {
        GameLog {
            messages: Vec::new(),
        }
    }

    /// Pushes the passed `message` to the [GameLog]'s message stream.
    /// 
    /// # Arguments
    /// * `message`: The message to add to the stream.
    /// 
    pub fn messages_push(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }

    /// Removes the passed `message` from the [GameLog]'s message
    /// stream.
    /// 
    /// # Arguments
    /// * `message`: The `message` to remove.
    /// 
    pub fn messages_remove(&mut self, message: &String) {
        self.messages.retain(|element| element != message);
    }

    /// Removes all messages from the [GameLog]'s stream.
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Iterates through all messages in the [GameLog]'s stream
    /// and executes the passed `block` with them.
    /// 
    /// # Arguments
    /// * `block`: The lambda to execute for each message.
    /// 
    pub fn messages_for_each_rev<F>(&mut self, mut block: F)
    where
        F: FnMut(&mut String),
    {
        for message in self.messages.iter_mut().rev() {
            block(message)
        }
    }
}

/// Struct to store the players `click-to-move` path
/// calculate through A*.
pub struct PlayerPathing {
    /// [Vec] containing the pathing information for players
    /// `click-to-move` movement.
    steps: Vec<usize>,
}

impl PlayerPathing {
    /// Creates a new [PlayerPathing] struct.
    pub fn new() -> Self {
        PlayerPathing { steps: Vec::new() }
    }

    /// Removes the first available entry form the player's
    /// pathing information and returns it. If there is no
    /// element in the pathing [Vec], the [Option] will contain
    /// [None].
    pub fn pop(&mut self) -> Option<usize> {
        self.steps.pop()
    }

    /// Overrides the current pathing [Vec] with the new
    /// information from the passed `steps` vec.
    /// 
    /// # Arguments
    /// * `steps`: The new pathing information.
    /// 
    pub fn update(&mut self, steps: &mut Vec<usize>) {
        self.steps.clear();
        self.steps.append(steps);
    }

    /// Removes all pathing information from the 
    /// [PlayerPathing]'s struct [Vec].
    pub fn clear(&mut self) {
        self.steps.clear();
    }
}
