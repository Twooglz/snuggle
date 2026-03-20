mod apple;
mod board;
mod direction;
mod snake;
mod vec2i;

use direction::{Direction, InputQueue};

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

fn main() {
    let mut stdout = io::stdout();

    enable_raw_mode().expect("Failed to enable raw mode");
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

    execute!(
        stdout,
        SetBackgroundColor(Color::Rgb { r: 0, g: 255, b: 0 }),
        Clear(ClearType::All)
    )
    .expect("color no worky");

    let tick_rate = Duration::from_millis(500);
    let mut input_queue = InputQueue::new();

    // Game loop
    'game_loop: loop {
        let now = Instant::now();

        // Register inputs
        loop {
            let time_until_tick: Duration = tick_rate.saturating_sub(now.elapsed());

            if time_until_tick == Duration::ZERO {
                break;
            }

            match read_key_events(time_until_tick) {
                Some(key) => {
                    match key.code {
                        KeyCode::Char('w') | KeyCode::Up => input_queue.add(Direction::Up),
                        KeyCode::Char('s') | KeyCode::Down => input_queue.add(Direction::Down),
                        KeyCode::Char('a') | KeyCode::Left => input_queue.add(Direction::Left),
                        KeyCode::Char('d') | KeyCode::Right => input_queue.add(Direction::Right),
                        KeyCode::Char('q') => break 'game_loop,
                        _ => continue,
                    }

                    if key.code == KeyCode::Char('q') {
                        break 'game_loop;
                    }
                }
                _ => continue,
            }

            writeln!(stdout, "Ran input loop, current queue: {:?}", input_queue);
        }
    }

    execute!(stdout, LeaveAlternateScreen).expect("Failed to leave alternate screen");
    disable_raw_mode().expect("Failed disabling raw mode...");
}

fn read_key_events(duration: Duration) -> Option<KeyEvent> {
    if poll(duration).ok()? {
        if let Event::Key(key) = read().ok()? {
            return Some(key);
        }
    }
    None
}
