mod room;
mod player;
mod game;
mod input;

use game::Game;
use input::{read_input, parse_command};

/// The main entry point for the game.
fn main() {
    // Display welcome message
    print_welcome();

    // Create a new game
    let mut game = Game::new();

    // Display initial room description
    println!("{}", game.look_around());
    println!();

    // Main game loop
    while !game.is_game_over() {
        // Read user input
        let user_input = read_input();

        // Parse the command
        match parse_command(&user_input) {
            Ok(command) => {
                // Process the command and get the result
                let result = game.process_command(command);

                // Display the result
                println!("{}", result);
                println!();
            },
            Err(error) => {
                // Display error message
                println!("{}",error);
                println!();
            }
        }
    }
}

/// Displays the welcome message and game title
fn print_welcome() {
    println!("=============================================");
    println!("|                                           |");
    println!("|         ESCAPE THE FORGOTTEN TEMPLE       |");
    println!("|             A Text Adventure              |");
    println!("|                                           |");
    println!("=============================================");
    println!();
    println!("You are an explorer who has ventured deep into a newly discovered ancient temple.");
    println!("While examining the inner chambers, a sudden tremor shakes the ground,");
    println!("causing a cave-in that blocks the entrance behind you.");
    println!("You must find another way out of this forgotten temple before it becomes your tomb.");
    println!();
    println!("Type 'help' for a list of commands.");
    println!();
}
