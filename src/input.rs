use crate::room::Direction;
use std::io::{self, Write};

/// Represents the possible commands a player can issue
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Move in a direction (e.g., "go north")
    Go(Direction),
    /// Pick up an item (e.g., "take key")
    Take(String),
    /// Use an item (e.g., "use key")
    Use(String),
    /// Display inventory (e.g., "inventory")
    Inventory,
    /// Look around the current room (e.g., "look")
    Look,
    /// Help command to show available commands (e.g., "help")
    Help,
    /// Quit the game (e.g., "quit")
    Quit,
    /// Unknown command
    Unknown(String),
}

/// Reads a line of input from the user
pub fn read_input() -> String {
    print!("> ");
    // Flush to ensure the prompt is displayed before reading input
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

/// Parses user input into a Command enum
pub fn parse_command(input: &str) -> Result<Command, String> {
    let input = input.trim().to_lowercase();

    if input.is_empty() {
        return Err("Please enter a command.".to_string());
    }

    // Split the input into words
    let mut words: Vec<&str> = input.split_whitespace().collect();

    if words.is_empty() {
        return Err("Please enter a command.".to_string());
    }

    let command = words[0];
    words.remove(0); // Remove the command, leaving only arguments

    match command {
        "go" | "move" => {
            if words.is_empty() {
                return Err("Go where? Try 'go north', 'go east', 'go south', or 'go west'.".to_string());
            }

            match Direction::from_string(words[0]) {
                Some(direction) => Ok(Command::Go(direction)),
                None => Err(format!("'{}' is not a valid direction. Try 'north', 'east', 'south', or 'west'.", words[0])),
            }
        },
        "take" | "get" | "pickup" => {
            if words.is_empty() {
                return Err("Take what? Please specify an item.".to_string());
            }

            Ok(Command::Take(words.join(" ")))
        },
        "use" => {
            if words.is_empty() {
                return Err("Use what? Please specify an item.".to_string());
            }

            Ok(Command::Use(words.join(" ")))
        },
        "inventory" | "i" | "inv" => {
            Ok(Command::Inventory)
        },
        "look" | "l" => {
            Ok(Command::Look)
        },
        "help" | "h" => {
            Ok(Command::Help)
        },
        "quit" | "exit" | "q" => {
            Ok(Command::Quit)
        },
        _ => {
            Ok(Command::Unknown(input))
        }
    }
}

/// Unit tests for the input module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_go_command() {
        assert_eq!(parse_command("go north"), Ok(Command::Go(Direction::North)));
        assert_eq!(parse_command("go east"), Ok(Command::Go(Direction::East)));
        assert_eq!(parse_command("move south"), Ok(Command::Go(Direction::South)));
        assert_eq!(parse_command("go west"), Ok(Command::Go(Direction::West)));

        // Case insensitivity
        assert_eq!(parse_command("Go North"), Ok(Command::Go(Direction::North)));

        // Invalid direction
        assert!(parse_command("go nowhere").is_err());

        // Missing direction
        assert!(parse_command("go").is_err());
    }

    #[test]
    fn test_parse_take_command() {
        assert_eq!(parse_command("take key"), Ok(Command::Take("key".to_string())));
        assert_eq!(parse_command("get torch"), Ok(Command::Take("torch".to_string())));
        assert_eq!(parse_command("take golden idol"), Ok(Command::Take("golden idol".to_string())));

        // Missing item
        assert!(parse_command("take").is_err());
    }

    #[test]
    fn test_parse_use_command() {
        assert_eq!(parse_command("use key"), Ok(Command::Use("key".to_string())));
        assert_eq!(parse_command("use golden idol"), Ok(Command::Use("golden idol".to_string())));

        // Missing item
        assert!(parse_command("use").is_err());
    }

    #[test]
    fn test_parse_inventory_command() {
        assert_eq!(parse_command("inventory"), Ok(Command::Inventory));
        assert_eq!(parse_command("inv"), Ok(Command::Inventory));
        assert_eq!(parse_command("i"), Ok(Command::Inventory));
    }

    #[test]
    fn test_parse_look_command() {
        assert_eq!(parse_command("look"), Ok(Command::Look));
        assert_eq!(parse_command("l"), Ok(Command::Look));
    }

    #[test]
    fn test_parse_help_command() {
        assert_eq!(parse_command("help"), Ok(Command::Help));
        assert_eq!(parse_command("h"), Ok(Command::Help));
    }

    #[test]
    fn test_parse_quit_command() {
        assert_eq!(parse_command("quit"), Ok(Command::Quit));
        assert_eq!(parse_command("exit"), Ok(Command::Quit));
        assert_eq!(parse_command("q"), Ok(Command::Quit));
    }

    #[test]
    fn test_parse_unknown_command() {
        assert_eq!(parse_command("jump"), Ok(Command::Unknown("jump".to_string())));
        assert_eq!(parse_command("dance"), Ok(Command::Unknown("dance".to_string())));
    }
}
