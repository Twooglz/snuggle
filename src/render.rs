use std::io::{Write, stdout};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
};

#[derive(Clone, Copy)]
pub struct Tile {
    value: [char; 2],
    foreground_color: Color,
    background_color: Color,
}

impl Tile {
    pub fn new(value: &str) -> Tile {
        let mut chars = value.chars();

        // not using the chars value because assert eats it
        assert!(value.chars().count() == 2);

        Self {
            value: [
                chars.next().expect("Missing first char"),
                chars.next().expect("Missing second char"),
            ],
            foreground_color: Color::Reset,
            background_color: Color::Reset,
        }
    }

    pub fn with_bg(mut self, color: Color) -> Tile {
        self.background_color = color;
        self
    }

    pub fn with_fg(mut self, color: Color) -> Tile {
        self.foreground_color = color;
        self
    }
}

pub struct RenderBuffer<const W: usize, const H: usize> {
    rows: [[Tile; W]; H],
}

impl<const W: usize, const H: usize> RenderBuffer<W, H> {
    pub fn new(tile: Tile) -> RenderBuffer<W, H> {
        RenderBuffer {
            rows: [[tile; W]; H],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Tile {
        assert!(x < W);
        assert!(y < H);

        self.rows[y][x]
    }

    pub fn set(&mut self, tile: Tile, x: usize, y: usize) {
        assert!(x < W);
        assert!(y < H);

        self.rows[y][x] = tile;
    }

    pub fn write(&self) {
        let mut stdout = stdout();
        let _ = execute!(stdout, BeginSynchronizedUpdate, MoveTo(0, 0));

        for y in 0..H {
            for x in 0..W {
                let tile = self.get(x, y); // char array
                let string: String = tile.value.iter().collect::<String>();

                execute!(
                    stdout,
                    SetForegroundColor(tile.foreground_color),
                    SetBackgroundColor(tile.background_color)
                )
                .expect("couldnt set colours!!!1111");

                let _ = write!(stdout, "{}", string);
            }
            let _ = write!(stdout, "\n\r");
        }

        let _ = execute!(stdout, EndSynchronizedUpdate);
    }
}
