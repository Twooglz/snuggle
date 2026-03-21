use crate::Direction;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub const UP: Self = Self { x: 0, y: -1 };
    pub const DOWN: Self = Self { x: 0, y: 1 }; // 0, 0 is top left, therefore y+ is down
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_direction(direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::UP,
            Direction::Down => Self::DOWN,
            Direction::Left => Self::LEFT,
            Direction::Right => Self::RIGHT,
        }
    }
}

impl Add for Vec2i {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Neg for Vec2i {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Vec2i {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl Mul<Vec2i> for i32 {
    type Output = Vec2i;

    fn mul(self, vec: Vec2i) -> Vec2i {
        Vec2i {
            x: vec.x * self,
            y: vec.y * self,
        }
    }
}
