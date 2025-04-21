# Escape the Forgotten Temple

A text-based adventure game built in Rust where you navigate through a forgotten temple, collect items, and solve puzzles to find your way out.

## Description

You are an explorer who has ventured deep into a newly discovered ancient temple. While examining the inner chambers, a sudden tremor shakes the ground, causing a cave-in that blocks the entrance behind you. You must find another way out of this forgotten temple before it becomes your tomb.

## Game Features

- Navigate through 6 unique temple rooms with detailed descriptions
- Collect and use items to progress through the game
- Text-based interface with intuitive commands
- Win by finding the temple exit with the required item

## How to Play

### Building the Game

To build the game, make sure you have Rust installed on your system, then run:

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

### Running the Game

To run the game:

```bash
cargo run
```

Or, after building, you can directly run the executable:

```bash
./target/debug/escape-forgotten-temple
```

Or for the release version:

```bash
./target/release/escape-forgotten-temple
```

### Commands

- `go [direction]`: Move in a direction (north, east, south, west)
- `take [item]`: Pick up an item
- `use [item]`: Use an item from your inventory
- `inventory`: View your inventory
- `look`: Look around the current room
- `help`: Display available commands
- `quit`: Exit the game

## Project Structure

- `main.rs`: Entry point and game loop
- `room.rs`: Room-related logic and data
- `player.rs`: Player state and actions
- `game.rs`: Core game logic and state management
- `input.rs`: Input parsing and command creation

## Game Map

The temple consists of the following rooms:

- Entrance Hall
- Ceremonial Antechamber
- Guardian Chamber (where you can find the golden idol)
- Treasure Room
- Ancient Crypt
- Temple Exit (requires the golden idol to escape)

## Development

This game was built using only Rust's standard library, with no external dependencies. It follows modern Rust coding practices and is designed to be modular and maintainable.

To run the tests:

```bash
cargo test
```

## License

This project is open source and available under the MIT License.
