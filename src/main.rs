extern crate framebuffer;
use framebuffer::{KdMode, Framebuffer};

mod math;
mod gfx;


fn main() {
    let mut game_screen = GameScreen::new();
    game_screen.draw();

    let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
    // TODO: Make variable that on destruction gets back to text mode
    let _ = std::io::stdin().read_line(&mut String::new());
    let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
}

struct PlayableArea {
    size: (u32, u32), // (width, height)
    location: (u32, u32), // (x, y)
}

struct GameScreen {
    arena: PlayableArea,
    canvas: gfx::Canvas,
}

impl GameScreen {
    fn new() -> GameScreen {
        GameScreen {
            arena: PlayableArea {
                size: (800, 600),
                location: (0, 0),
            },
            canvas: gfx::Canvas::new(),
        }
    }

    fn draw(&mut self) {
        self.draw_arena();
        self.canvas.sprite_to(500, 200, gfx::Sprite::head());
        self.canvas.sprite_to(525, 200, gfx::Sprite::body());
        self.canvas.draw();
    }

    fn draw_arena(&mut self) {
        self.canvas.color(255, 255, 255);
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
