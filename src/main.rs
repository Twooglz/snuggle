mod apple;
mod board;
mod direction;
mod input;
mod render;
mod snake;
mod vec2i;

use direction::Direction;
use input::InputHandler;

use crossterm::{
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use std::io;
use std::time::Duration;

use crate::board::Board;

fn main() {
    let mut stdout = io::stdout();

    enable_raw_mode().expect("Failed to enable raw mode");
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

    let tick_duration = Duration::from_millis(250);
    let mut input_handler = InputHandler::new();

    let mut board = Board::<20, 20>::new();
    let _ = execute!(stdout, Clear(ClearType::All));

    // Game loop
    'game_loop: loop {
        board.render_buffer().write();

        input_handler.read_inputs(tick_duration);

        if input_handler.exit_pressed() {
            break 'game_loop;
        }

        board.update(&mut input_handler);
    }

    execute!(stdout, LeaveAlternateScreen).expect("Failed to leave alternate screen");
    disable_raw_mode().expect("Failed disabling raw mode...");
}
