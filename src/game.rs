use std::collections::HashMap;
use crate::room::{Room, Direction, create_rooms};
use crate::player::Player;
use crate::input::Command;

/// Game state and logic
pub struct Game {
    /// All rooms in the game
    rooms: HashMap<String, Room>,
    /// The player
    player: Player,
    /// Flag indicating if the game is over
    game_over: bool,
    /// Game messages to display
    message: String,
}

impl Game {
    /// Create a new game with the starting room
    pub fn new() -> Self {
        let rooms = create_rooms();
        let player = Player::new("Entrance Hall");

        Game {
            rooms,
            player,
            game_over: false,
            message: String::new(),
        }
    }

    /// Process a command and update the game state
    pub fn process_command(&mut self, command: Command) -> String {
        match command {
            Command::Go(direction) => self.handle_go(direction),
            Command::Take(item) => self.handle_take(&item),
            Command::Use(item) => self.handle_use(&item),
            Command::Inventory => self.player.display_inventory(),
            Command::Look => self.look_around(),
            Command::Help => self.display_help(),
            Command::Quit => {
                self.game_over = true;
                "Thanks for playing! Goodbye.".to_string()
            },
            Command::Unknown(input) => format!("I don't understand '{}'.\nType 'help' for a list of commands.", input),
        }
    }

    /// Handle the 'go' command
    fn handle_go(&mut self, direction: Direction) -> String {
        // Get the current room
        if let Some(current_room) = self.rooms.get(&self.player.location) {
            // Check if the direction is valid
            if let Some(next_room_name) = current_room.exits.get(&direction) {
                // Move the player to the next room
                self.player.location = next_room_name.clone();

                // Check if this is the exit room and if the player has the required item
                self.check_win_condition();

                // Return the description of the new room
                self.look_around()
            } else {
                format!("You can't go {} from here.", direction.to_string())
            }
        } else {
            "Error: Current room not found.".to_string()
        }
    }

    /// Handle the 'take' command
    fn handle_take(&mut self, item: &str) -> String {
        // Get the current room
        if let Some(current_room) = self.rooms.get_mut(&self.player.location) {
            // Check if the item is in the room
            if current_room.remove_item(item) {
                // Add the item to the player's inventory
                self.player.take_item(item);
                format!("You take the {}.", item)
            } else {
                format!("There is no {} here.", item)
            }
        } else {
            "Error: Current room not found.".to_string()
        }
    }

    /// Handle the 'use' command
    fn handle_use(&mut self, item: &str) -> String {
        // Check if the player has the item
        if self.player.has_item(item) {
            // Get the current room
            if let Some(current_room) = self.rooms.get(&self.player.location) {
                // Special item interactions based on the room and item
                match (current_room.name.as_str(), item) {
                    ("Temple Exit", "golden idol") => {
                        self.game_over = true;
                        "You place the golden idol in the keyhole. With a rumble, the stone doors slowly open, \
                        revealing the path to freedom. Sunlight streams in, blinding you momentarily. \
                        \n\nCongratulations! You have escaped the forgotten temple!".to_string()
                    },
                    ("Ancient Crypt", "torch") => {
                        "You light the torch. The crypt is now illuminated, revealing ancient inscriptions \
                        on the walls that were previously hidden in darkness.".to_string()
                    },
                    ("Entrance Hall", "ancient map") => {
                        "You examine the ancient map. It shows the layout of the temple, confirming \
                        your suspicions about the locations of the rooms. The exit appears to be \
                        north of the Treasure Room.".to_string()
                    },
                    ("Ceremonial Antechamber", "ceremonial dagger") => {
                        "You place the ceremonial dagger on the altar. Nothing happens, but you feel \
                        a sense of respect for the ancient rituals once performed here.".to_string()
                    },
                    _ => format!("You can't use the {} here.", item),
                }
            } else {
                "Error: Current room not found.".to_string()
            }
        } else {
            format!("You don't have a {}.", item)
        }
    }

    /// Check if the player has won the game
    fn check_win_condition(&mut self) {
        if let Some(current_room) = self.rooms.get(&self.player.location) {
            if current_room.is_exit {
                if let Some(required_item) = &current_room.required_item {
                    if self.player.has_item(required_item) {
                        self.message = format!(
                            "You've reached the exit with the {}! Use the item to escape.",
                            required_item
                        );
                    } else {
                        self.message = format!(
                            "This appears to be an exit, but it's blocked. You need a {} to proceed.",
                            required_item
                        );
                    }
                }
            }
        }
    }

    /// Look around the current room
    pub fn look_around(&self) -> String {
        // Get the current room
        if let Some(current_room) = self.rooms.get(&self.player.location) {
            let mut description = format!("[ {} ]\n\n{}\n", current_room.name, current_room.description);

            // Add exits
            if !current_room.exits.is_empty() {
                description.push_str("\nExits:");
                for (direction, _) in &current_room.exits {
                    description.push_str(&format!(" {}", direction.to_string()));
                }
            }

            // Add items
            if !current_room.items.is_empty() {
                description.push_str("\n\nYou see:");
                for item in &current_room.items {
                    description.push_str(&format!("\n- {}", item));
                }
            }

            // Add any special messages
            if !self.message.is_empty() {
                description.push_str(&format!("\n\n{}", self.message));
            }

            description
        } else {
            "Error: Current room not found.".to_string()
        }
    }

    /// Display help text
    fn display_help(&self) -> String {
        "Available commands:\n\
        - go [direction]: Move in the specified direction (north, east, south, west)\n\
        - take [item]: Pick up an item\n\
        - use [item]: Use an item from your inventory\n\
        - look: Look around the current room\n\
        - inventory: Check your inventory\n\
        - help: Display this help text\n\
        - quit: Exit the game".to_string()
    }

    /// Check if the game is over
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{Command, parse_command};

    #[test]
    fn test_game_initialization() {
        let game = Game::new();
        assert_eq!(game.player.location, "Entrance Hall");
        assert_eq!(game.player.inventory.len(), 0);
        assert_eq!(game.game_over, false);
    }

    #[test]
    fn test_process_go_command() {
        let mut game = Game::new();
        let result = game.process_command(Command::Go(Direction::North));
        assert_eq!(game.player.location, "Ceremonial Antechamber");
        assert!(result.contains("Ceremonial Antechamber"));

        // Try an invalid direction
        let result = game.process_command(Command::Go(Direction::North));
        assert_eq!(game.player.location, "Ceremonial Antechamber"); // Location shouldn't change
        assert!(result.contains("can't go"));
    }

    #[test]
    fn test_take_item() {
        let mut game = Game::new();
        let result = game.process_command(Command::Take("ancient map".to_string()));
        assert!(game.player.inventory.contains(&"ancient map".to_string()));
        assert!(result.contains("You take"));

        // Try taking a nonexistent item
        let result = game.process_command(Command::Take("gold coin".to_string()));
        assert!(!game.player.inventory.contains(&"gold coin".to_string()));
        assert!(result.contains("There is no"));
    }
}
