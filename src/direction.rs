#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug)]
pub struct InputQueue {
    queue: Vec<Direction>,
}

// first in first out
impl InputQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn add(&mut self, direction: Direction) {
        if self.queue.contains(&direction) {
            return;
        }
        self.queue.insert(0, direction);
    }

    pub fn pop(&mut self) -> Option<Direction> {
        self.queue.pop()
    }
}
