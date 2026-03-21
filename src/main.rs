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
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute,
    style::{Color, SetBackgroundColor},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use std::io;
use std::io::Write;
use std::time::{Duration, Instant};

use crate::render::{RenderBuffer, Tile};

fn main() {
    let mut stdout = io::stdout();

    enable_raw_mode().expect("Failed to enable raw mode");
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

    let tick_rate = Duration::from_millis(500);
    let mut input_handler = InputHandler::new();

    // Game loop
    'game_loop: loop {
        let _ = execute!(stdout, Clear(ClearType::All),);

        let mut render_buffer = RenderBuffer::<10, 10>::new(Tile::new("..").with_fg(Color::Blue));

        for x in 2..=6 {
            render_buffer.set(Tile::new("[]").with_fg(Color::Green), x, 2);
        }

        render_buffer.set(Tile::new("[]").with_fg(Color::Red), 5, 5);

        render_buffer.write_buffer();

        input_handler.read_inputs(Duration::from_millis(500));

        if input_handler.exit_pressed() {
            break 'game_loop;
        }

        // // Register inputs
        // loop {
        //     let time_until_tick: Duration = tick_rate.saturating_sub(now.elapsed());

        //     if time_until_tick <= Duration::ZERO {
        //         break;
        //     }

        //     match read_key_events(time_until_tick) {
        //         Some(key) => {
        //             match key.code {
        //                 KeyCode::Char('w') | KeyCode::Up => input_queue.add(Direction::Up),
        //                 KeyCode::Char('s') | KeyCode::Down => input_queue.add(Direction::Down),
        //                 KeyCode::Char('a') | KeyCode::Left => input_queue.add(Direction::Left),
        //                 KeyCode::Char('d') | KeyCode::Right => input_queue.add(Direction::Right),
        //                 KeyCode::Char('q') => break 'game_loop,
        //                 _ => continue,
        //             }

        //             if key.code == KeyCode::Char('q') {
        //                 break 'game_loop;
        //             }
        //         }
        //         _ => continue,
        //     }
        // }
    }

    execute!(stdout, LeaveAlternateScreen).expect("Failed to leave alternate screen");
    disable_raw_mode().expect("Failed disabling raw mode...");
}

// fn read_key_events(duration: Duration) -> Option<KeyEvent> {
//     if poll(duration).ok()? {
//         if let Event::Key(key) = read().ok()? {
//             return Some(key);
//         }
//     }
//     None
// }
