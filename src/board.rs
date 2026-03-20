use std::collections::HashSet;

use rand::seq::IteratorRandom;

use crate::apple::Apple;
use crate::direction::{Direction, InputQueue};
use crate::snake::Snake;
use crate::vec2i::Vec2i;

pub struct Board {
    snake: Snake,
    apples: HashSet<Apple>,
    size: Vec2i,
    free_spaces: HashSet<Vec2i>,
}

impl Board {
    pub fn new(size_x: i32, size_y: i32) -> Board {
        let snake_pos = Vec2i::new(
            5, // maybe i'll make some actual formula one day, this'll do for now though
            size_y / 2 + 1,
        );

        let mut free_spaces: HashSet<Vec2i> = HashSet::new();

        for x in 0..size_x {
            for y in 0..size_y {
                free_spaces.insert(Vec2i::new(x, y));
            }
        }

        let snake = Snake::new(snake_pos, 3, Direction::Right);
        let apples = HashSet::from([Apple {
            position: Vec2i::new(size_x - 3, size_y / 2 + 1),
        }]);

        for segment in &snake.segments {
            free_spaces.remove(&segment.position);
        }

        for apple in &apples {
            free_spaces.remove(&apple.position);
        }

        Board {
            snake,
            apples,
            size: Vec2i::new(size_x, size_y),
            free_spaces,
        }
    }

    pub fn tick(&mut self, input_queue: &mut InputQueue) {
        let snake = &self.snake;
        let direction: Direction = match input_queue.pop() {
            Some(input) if input != snake.facing() => input,
            Some(_) => input_queue.pop().unwrap_or(snake.facing()),
            // the input is the same direction we're facing, do the next input, if that's None just set the direction to where snakey's looking
            // InputQueue has unique elements, so if the next input isn't None it's valid.
            None => snake.facing(),
        };

        let next_head_pos = snake.head().position + Vec2i::from_direction(direction);

        let grow = if let Some(apple) = self.apple_at(next_head_pos) {
            self.apples.remove(&apple);
            self.place_apple();
            true
        } else {
            self.free_spaces.remove(&self.snake.tail().position);
            false
        };

        self.free_spaces.remove(&next_head_pos);
        let snake = &mut self.snake;
        snake.slither(direction, grow);
    }

    pub fn apple_at(&self, position: Vec2i) -> Option<Apple> {
        for apple in &self.apples {
            if apple.position == position {
                return Some(apple.clone());
            }
        }
        None
    }

    pub fn place_apple(&self) {
        let mut rng = rand::rng();
        self.free_spaces.iter().choose(&mut rng);
    }
}
