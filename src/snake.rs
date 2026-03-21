use crate::direction::Direction;
use crate::vec2i::Vec2i;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SnakeSegment {
    pub position: Vec2i,
    pub direction: Direction,
}

pub struct Snake {
    // Head first, tail last
    pub segments: Vec<SnakeSegment>,
}

impl Snake {
    pub fn new(pos: Vec2i, length: i32, direction: Direction) -> Self {
        let direction_vec = Vec2i::from_direction(direction);
        let mut segments: Vec<SnakeSegment> = Vec::new();

        for i in 0..length {
            segments.push(SnakeSegment {
                position: pos - i * direction_vec,
                direction,
            });
        }

        Snake { segments }
    }

    pub fn head(&self) -> &SnakeSegment {
        self.segments.first().expect("Snake is empty")
    }

    pub fn tail(&self) -> &SnakeSegment {
        self.segments.last().expect("Snake is empty")
    }

    pub fn facing(&self) -> Direction {
        self.head().direction
    }

    pub fn set_facing(&mut self, direction: Direction) {
        let head = self.segments.first_mut().expect("Snake is empty");

        head.direction = direction;
    }

    pub fn next_head_pos(&self) -> Vec2i {
        let head = self.head();

        head.position + Vec2i::from_direction(head.direction)
    }

    // would've used "move" but that's taken
    pub fn slither(&mut self, grow: bool) {
        // remove tail, when not "growing", if we do this and add a new head it's like it's moving.
        // when growing this popping the tail isn't needed.
        if !grow {
            self.segments.pop();
        }
        // new head
        self.segments.insert(
            0,
            SnakeSegment {
                position: self.head().position + Vec2i::from_direction(self.facing()),
                direction: self.facing(),
            },
        );
    }
}
