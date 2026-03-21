use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode, KeyEvent, poll, read};

use crate::direction::Direction;

#[derive(Debug)]
pub struct InputHandler {
    queue: Vec<Direction>,
    exit: bool,
}

// first in first out
impl InputHandler {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            exit: false,
        }
    }

    pub fn queue_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn queue_add(&mut self, direction: Direction) {
        if self.queue.contains(&direction) {
            return;
        }
        self.queue.insert(0, direction);
    }

    pub fn pop(&mut self) -> Option<Direction> {
        self.queue.pop()
    }

    pub fn read_inputs(&mut self, duration: Duration) {
        let read_start = Instant::now();
        // Register inputs
        loop {
            let time_until_tick: Duration = duration.saturating_sub(read_start.elapsed());

            if time_until_tick <= Duration::ZERO {
                return;
            }

            match read_key_events(time_until_tick) {
                Some(key) => match key.code {
                    KeyCode::Char('w') | KeyCode::Up => self.queue_add(Direction::Up),
                    KeyCode::Char('s') | KeyCode::Down => self.queue_add(Direction::Down),
                    KeyCode::Char('a') | KeyCode::Left => self.queue_add(Direction::Left),
                    KeyCode::Char('d') | KeyCode::Right => self.queue_add(Direction::Right),
                    KeyCode::Char('q') => {
                        self.exit = true;
                        return;
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }
    }

    pub fn exit_pressed(&self) -> bool {
        self.exit
    }
}

fn read_key_events(duration: Duration) -> Option<KeyEvent> {
    if poll(duration).ok()? {
        if let Event::Key(key) = read().ok()? {
            return Some(key);
        }
    }
    None
}
