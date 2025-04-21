use druid::{
    widget::{Button, Container, Flex, Label, TextBox, CrossAxisAlignment},
    Data, Lens, Widget, WidgetExt, Color,
    keyboard_types::Key,
    EventCtx, Event, KeyOrValue
};
use crate::game::Game;
use crate::room::Direction;
use crate::input::{Command, parse_command};

// Constants for UI sizing and styling
pub const WINDOW_TITLE: &str = "Escape the Forgotten Temple";
const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;
const PADDING: f64 = 8.0;
const BUTTON_WIDTH: f64 = 100.0;
const BUTTON_HEIGHT: f64 = 40.0;

// Custom colors for temple theme
const TEMPLE_BACKGROUND: Color = Color::rgb8(35, 31, 32);
const TEMPLE_TEXT: Color = Color::rgb8(255, 248, 231);
const TEMPLE_BUTTON: Color = Color::rgb8(139, 69, 19);
const TEMPLE_BUTTON_HOVER: Color = Color::rgb8(160, 82, 45);

#[derive(Clone, Data, Lens)]
pub struct UiState {
    input_text: String,
    feedback_text: String,
    #[data(ignore)]
    game: Game,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            input_text: String::new(),
            feedback_text: String::from("Welcome to the Forgotten Temple! Type 'help' for commands."),
            game: Game::new(),
        }
    }

    pub fn process_input(&mut self) {
        if self.input_text.is_empty() {
            return;
        }

        match parse_command(&self.input_text) {
            Ok(cmd) => {
                self.feedback_text = self.game.process_command(cmd);
            }
            Err(error) => {
                self.feedback_text = error;
            }
        }
        self.input_text.clear();
    }

    pub fn handle_direction(&mut self, direction: Direction) {
        self.feedback_text = self.game.process_command(Command::Go(direction));
    }

    pub fn handle_take(&mut self, item: String) {
        self.feedback_text = self.game.process_command(Command::Take(item));
    }

    pub fn handle_use(&mut self, item: String) {
        self.feedback_text = self.game.process_command(Command::Use(item));
    }

    pub fn handle_look(&mut self) {
        self.feedback_text = self.game.process_command(Command::Look);
    }

    pub fn handle_help(&mut self) {
        self.feedback_text = self.game.process_command(Command::Help);
    }
}

struct TextBoxController;

impl<W: Widget<UiState>> druid::widget::Controller<UiState, W> for TextBoxController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut UiState, env: &druid::Env) {
        if let Event::KeyDown(key_event) = event {
            if key_event.key == Key::Enter {
                data.process_input();
                ctx.request_update();
            }
        }
        child.event(ctx, event, data, env)
    }
}

pub fn build_ui() -> impl Widget<UiState> {
    // Room description area with temple styling
    let room_description = Label::dynamic(|data: &UiState, _| {
        data.game.get_current_room_description()
    })
    .with_text_size(18.0)
    .with_text_color(TEMPLE_TEXT)
    .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
    .center()
    .padding(PADDING)
    .background(TEMPLE_BACKGROUND)
    .rounded(8.0);

    // Direction buttons in a cross layout
    let direction_buttons = Flex::column()
        .with_child(
            Button::new("North")
                .on_click(|_ctx, data: &mut UiState, _env| data.handle_direction(Direction::North))
                .fix_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        )
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("West")
                        .on_click(|_ctx, data: &mut UiState, _env| data.handle_direction(Direction::West))
                        .fix_size(BUTTON_WIDTH, BUTTON_HEIGHT)
                )
                .with_spacer(BUTTON_WIDTH)
                .with_child(
                    Button::new("East")
                        .on_click(|_ctx, data: &mut UiState, _env| data.handle_direction(Direction::East))
                        .fix_size(BUTTON_WIDTH, BUTTON_HEIGHT)
                )
        )
        .with_child(
            Button::new("South")
                .on_click(|_ctx, data: &mut UiState, _env| data.handle_direction(Direction::South))
                .fix_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        )
        .cross_axis_alignment(CrossAxisAlignment::Center);

    // Action buttons
    let action_buttons = Flex::row()
        .with_child(
            Button::new("Look")
                .on_click(|_ctx, data: &mut UiState, _env| data.handle_look())
                .fix_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        )
        .with_spacer(PADDING)
        .with_child(
            Button::new("Help")
                .on_click(|_ctx, data: &mut UiState, _env| data.handle_help())
                .fix_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        );

    // Items in room buttons
    let room_items = Flex::row()
        .with_child(
            Label::new("Items in room:")
                .with_text_color(TEMPLE_TEXT)
        )
        .with_flex_child(
            Flex::row()
                .with_child(
                    Container::new(
                        Label::dynamic(|data: &UiState, _| {
                            let items = data.game.get_room_items();
                            if items.is_empty() {
                                "None".to_string()
                            } else {
                                items.join(", ")
                            }
                        })
                        .with_text_color(TEMPLE_TEXT)
                    )
                    .background(TEMPLE_BACKGROUND)
                    .rounded(4.0)
                    .padding(4.0)
                ),
            1.0,
        );

    // Feedback area with scrolling
    let feedback = Container::new(
        Label::dynamic(|data: &UiState, _| data.feedback_text.clone())
            .with_text_size(14.0)
            .with_text_color(TEMPLE_TEXT)
            .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
    )
    .background(TEMPLE_BACKGROUND)
    .rounded(8.0)
    .padding(PADDING);

    // Input area
    let input = TextBox::new()
        .with_placeholder("Enter command...")
        .lens(UiState::input_text)
        .fix_width(400.0)
        .controller(TextBoxController);

    // Inventory display
    let inventory = Container::new(
        Label::dynamic(|data: &UiState, _| {
            format!("Inventory: {}", data.game.get_inventory_display())
        })
        .with_text_color(TEMPLE_TEXT)
    )
    .background(TEMPLE_BACKGROUND)
    .rounded(4.0)
    .padding(PADDING);

    // Main layout
    Container::new(
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(room_description)
            .with_spacer(PADDING)
            .with_child(direction_buttons)
            .with_spacer(PADDING)
            .with_child(room_items)
            .with_spacer(PADDING)
            .with_child(inventory)
            .with_spacer(PADDING)
            .with_child(action_buttons)
            .with_spacer(PADDING)
            .with_child(feedback)
            .with_spacer(PADDING)
            .with_child(input)
    )
    .background(Color::rgb8(48, 43, 39))
    .padding(PADDING)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_state_initialization() {
        let state = UiState::new();
        assert!(state.feedback_text.contains("Welcome"));
        assert_eq!(state.input_text, "");
    }

    #[test]
    fn test_handle_direction() {
        let mut state = UiState::new();
        state.handle_direction(Direction::North);
        assert!(state.feedback_text.contains("Ceremonial Antechamber"));
    }

    #[test]
    fn test_process_input() {
        let mut state = UiState::new();
        state.input_text = "look".to_string();
        state.process_input();
        assert!(state.feedback_text.contains("Entrance Hall"));
        assert_eq!(state.input_text, "");
    }

    #[test]
    fn test_help_command() {
        let mut state = UiState::new();
        state.handle_help();
        assert!(state.feedback_text.contains("Available commands"));
    }
}
