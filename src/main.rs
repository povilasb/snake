extern crate framebuffer;
use framebuffer::{KdMode, Framebuffer};

mod math;
mod gfx;


fn main() {
    let mut canvas = gfx::Canvas::new();
    canvas.sprite_to(500, 200, gfx::Sprite::head());
    canvas.sprite_to(525, 200, gfx::Sprite::body());
    canvas.draw();

    let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
    // TODO: Make variable that on destruction gets back to text mode
    let _ = std::io::stdin().read_line(&mut String::new());
    let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
}
