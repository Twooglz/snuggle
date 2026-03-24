use crossterm::style::Color;
use rand::seq::IteratorRandom;
use std::collections::HashSet;

use crate::apple::Apple;
use crate::direction::Direction;
use crate::input::InputHandler;
use crate::render::{RenderBuffer, Tile};
use crate::snake::Snake;
use crate::vec2i::Vec2i;

pub struct Board<const W: usize, const H: usize> {
    snake: Snake,
    apples: HashSet<Apple>,
    free_spaces: HashSet<Vec2i>,
    fail: bool,
}

impl<const W: usize, const H: usize> Board<W, H> {
    pub fn new() -> Self {
        let snake_pos = Vec2i::new(
            5, // maybe i'll make some actual formula one day, this'll do for now though
            H as i32 / 2 + 1,
        );
        let mut free_spaces: HashSet<Vec2i> = HashSet::new();

        for x in 0..W {
            for y in 0..H {
                free_spaces.insert(Vec2i::new(x as i32, y as i32));
            }
        }

        let snake = Snake::new(snake_pos, 3, Direction::Right);
        let apples = HashSet::from([Apple {
            position: Vec2i::new(W as i32 - 3, H as i32 / 2 + 1),
        }]);

        for segment in &snake.segments {
            free_spaces.remove(&segment.position);
        }

        for apple in &apples {
            free_spaces.remove(&apple.position);
        }

        Board::<W, H> {
            snake,
            apples,
            free_spaces,
            fail: false,
        }
    }

    pub fn failed(&mut self) -> bool {
        self.fail
    }

    pub fn update(&mut self, input_handler: &mut InputHandler) {
        {
            let snake = &mut self.snake;

            let direction: Direction = loop {
                if input_handler.queue_empty() {
                    break snake.facing();
                }
                let input = input_handler.pop().expect("something broke");

                if input == snake.facing() || input == snake.facing().opposite() {
                    continue;
                }

                break input;
            };

            snake.set_facing(direction);
        }

        let next_head_pos = self.snake.next_head_pos();

        let grow = if let Some(apple) = self.apple_at(next_head_pos) {
            self.apples.remove(&apple);
            self.place_apple();
            true
        } else {
            if !self.free_spaces.contains(&next_head_pos) {
                self.fail = true;
            }

            self.free_spaces.insert(self.snake.tail().position.clone());
            false
        };

        let snake = &mut self.snake;
        snake.slither(grow);

        self.free_spaces.remove(&next_head_pos);
    }

    pub fn apple_at(&self, position: Vec2i) -> Option<Apple> {
        for apple in &self.apples {
            if apple.position == position {
                return Some(apple.clone());
            }
        }
        None
    }

    pub fn place_apple(&mut self) {
        let mut rng = rand::rng();
        let space = *self
            .free_spaces
            .iter()
            .choose(&mut rng)
            .expect("Couldn't place apple!");
        self.apples.insert(Apple { position: space });
        self.free_spaces.remove(&space);
    }

    pub fn render_buffer(&self) -> RenderBuffer<W, H> {
        let mut render_buffer = RenderBuffer::<W, H>::new(Tile::new("..").with_fg(Color::Blue));

        for segment in &self.snake.segments {
            let x: usize = segment.position.x as usize;
            let y: usize = segment.position.y as usize;

            render_buffer.set(Tile::new("[]").with_fg(Color::Green), x, y)
        }

        for apple in &self.apples {
            let x: usize = apple.position.x as usize;
            let y: usize = apple.position.y as usize;

            render_buffer.set(Tile::new("[]").with_fg(Color::Red), x, y)
        }

        // This renders free spaces as yellow
        // for x in 0..W {
        //     for y in 0..H {
        //         if self.free_spaces.contains(&Vec2i::new(x as i32, y as i32)) {
        //             render_buffer.set(render_buffer.get(x, y).with_fg(Color::Yellow), x, y)
        //         }
        //     }
        // }

        render_buffer
    }
}
