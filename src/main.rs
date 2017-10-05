extern crate framebuffer;

use std::thread;
use std::time::Duration;

use framebuffer::{KdMode, Framebuffer};

fn main() {
    let mut canvas = Canvas::new();
    canvas.line((100, 100), (150, 100));
    canvas.draw();

    let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
    // TODO: Make variable that on destruction gets back to text mode
    thread::sleep(Duration::from_secs(5));
    let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
}

/// (x, y)
type Point = (u32, u32);

struct Canvas {
    fb: Framebuffer,
    width: u32,
    height: u32,
    line_length: u32,
    frame: Vec<u8>,
}

impl Canvas {
    fn new() -> Canvas {
        let fb = Framebuffer::new("/dev/fb0").expect("Failed to open fb0");
        let line_length = fb.fix_screen_info.line_length;
        let height = fb.var_screen_info.yres;
        Canvas {
            width: fb.var_screen_info.xres,
            height,
            frame: vec![0; (line_length * height) as usize],
            line_length,
            fb,
        }
    }

    // only horizontal lines
    fn line(&mut self, from: Point, to: Point) {
        let color = (255, 255, 0);
        for x in from.0..to.0 {
            let start = (to.1 * self.line_length + x) as usize;
            self.frame[start]  = color.0;
            self.frame[start + 1]  = color.1;
            self.frame[start + 2]  = color.2;
        }
    }

    fn draw(&mut self) {
        self.fb.write_frame(&self.frame);
    }
}
