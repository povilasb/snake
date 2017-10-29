#[macro_use]
extern crate unwrap;

use std::{thread, time, io};
use std::io::Read;
use std::collections::HashMap;

extern crate framebuffer;
use framebuffer::{KdMode, Framebuffer};
extern crate termion;
use termion::raw::IntoRawMode;
use termion::async_stdin;

#[cfg(test)] #[macro_use] extern crate hamcrest;

mod math;
mod gfx;
mod game;

use game::MovementDirection;

fn main() {
    let _ = unwrap!(Framebuffer::set_kd_mode(KdMode::Graphics));

    // This also puts stdin to raw mode which allows to get unbuffered key
    // presses.
    let stdout = io::stdout();
    unwrap!(stdout.lock().into_raw_mode());

    let mut plane = game::Plane::new(32, 24);
    let mut game_screen = GameScreen::new();
    let mut stdin = async_stdin();

    let delay = time::Duration::from_millis(150);
    let mut key_bytes = [0];

    let mut direction = MovementDirection::Down;
    loop {
        unwrap!(stdin.read(&mut key_bytes));
        match key_bytes[0] {
            b'l' => direction = MovementDirection::Right,
            b'h' => direction = MovementDirection::Left,
            b'k' => direction = MovementDirection::Up,
            b'j' => direction = MovementDirection::Down,
            _ => (),
        };
        if key_bytes[0] == b'q' {
            break;
        }
        plane.move_to(&direction);

        game_screen.draw(&plane);
        thread::sleep(delay);
    }

    unwrap!(Framebuffer::set_kd_mode(KdMode::Text));
}

struct PlayableArea {
    size: (u32, u32), // (width, height)
    location: (u32, u32), // (x, y)
}

struct GameScreen {
    arena: PlayableArea,
    canvas: gfx::Canvas,
    cell_size: (u32, u32),
    head_sprites: HashMap<MovementDirection, gfx::Sprite>,
}

impl GameScreen {
    fn new() -> GameScreen {
        let mut head_sprites = HashMap::new();
        head_sprites.insert(MovementDirection::Left, gfx::Sprite::head_left());
        head_sprites.insert(MovementDirection::Right, gfx::Sprite::head_right());
        head_sprites.insert(MovementDirection::Up, gfx::Sprite::head_up());
        head_sprites.insert(MovementDirection::Down, gfx::Sprite::head_down());
        GameScreen {
            arena: PlayableArea {
                size: (800, 600),
                location: (0, 0),
            },
            canvas: gfx::Canvas::new(),
            cell_size: (25, 25),
            head_sprites,
        }
    }

    fn draw(&mut self, plane: &game::Plane) {
        self.canvas.clear();
        self.draw_arena();
        self.draw_snake(plane);
        self.canvas.draw();
    }

    fn draw_snake(&mut self, plane: &game::Plane) {
        let (x, y) = self.arena.location;
        let head = &plane.snake[0];
        self.canvas.sprite_to(
            x + self.cell_size.0 * head.x as u32,
            y + self.cell_size.1 * head.y as u32,
            unwrap!(self.head_sprites.get(&head.direction)),
        );
        for cell in plane.snake.iter().skip(1) {
            self.canvas.sprite_to(
                x + self.cell_size.0 * cell.x as u32,
                y + self.cell_size.1 * cell.y as u32,
                &gfx::Sprite::body()
            );
        }
    }

    fn draw_arena(&mut self) {
        self.canvas.color(66, 134, 244);
        let (w, h) = self.arena.size;
        let (x, y) = self.arena.location;
        // horizontal lines
        self.canvas.line((x, y), (x + w - 1, y));
        self.canvas.line((x, y + h - 1), (x + w - 1, y + h - 1));
        // vertical lines
        self.canvas.vline((x, y), (x, y + h - 1));
        self.canvas.vline((x + w - 1, y), (x + w - 1, y + h - 1));
    }
}
