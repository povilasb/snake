extern crate framebuffer;
use framebuffer::{KdMode, Framebuffer};

mod math;
mod gfx;
mod game;


fn main() {
    let mut plane = game::Plane::new(32, 24);
    let mut game_screen = GameScreen::new();
    game_screen.draw(&plane);

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
    cell_size: (u32, u32),
}

impl GameScreen {
    fn new() -> GameScreen {
        GameScreen {
            arena: PlayableArea {
                size: (800, 600),
                location: (0, 0),
            },
            canvas: gfx::Canvas::new(),
            cell_size: (25, 25)
        }
    }

    fn draw(&mut self, plane: &game::Plane) {
        self.draw_arena();
        let (x, y) = self.arena.location;
        let head = &plane.snake[0];
        self.canvas.sprite_to(
            x + self.cell_size.0 * head.x as u32,
            y + self.cell_size.1 * head.y as u32,
            gfx::Sprite::head()
        );
        for cell in plane.snake.iter().skip(1) {
            self.canvas.sprite_to(
                x + self.cell_size.0 * cell.x as u32,
                y + self.cell_size.1 * cell.y as u32,
                gfx::Sprite::body()
            );
        }
        self.canvas.draw();
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
