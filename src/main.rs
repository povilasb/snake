extern crate framebuffer;
use framebuffer::{KdMode, Framebuffer};

mod math;
mod gfx;

use std::thread;
use std::time::Duration;


fn main() {
    let mut canvas = gfx::Canvas::new();
    canvas.line((100, 100), (150, 100));
    canvas.line((100, 200), (500, 500));
    canvas.color(255, 0, 0);
    canvas.rect((500, 0), (550, 50));
    canvas.draw();

    let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
    // TODO: Make variable that on destruction gets back to text mode
    thread::sleep(Duration::from_secs(5));
    let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
}
