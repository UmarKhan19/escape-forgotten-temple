mod room;
mod player;
mod game;
mod input;
mod ui;

use druid::{AppLauncher, WindowDesc};
use ui::{UiState, build_ui, WINDOW_TITLE};

/// The main entry point for the game.
fn main() {
    // Create the main window
    let main_window = WindowDesc::new(build_ui())
        .title(WINDOW_TITLE)
        .window_size((800.0, 600.0));

    // Create the initial game state
    let initial_state = UiState::new();

    // Launch the app
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}
