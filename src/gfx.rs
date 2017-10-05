/// Graphics utils

use framebuffer::Framebuffer;

use math;

/// (x, y)
type Point = (u32, u32);

/// Represents 2D drawing area.
/// Draws directly to Linux framebuffer.
/// See: https://www.kernel.org/doc/Documentation/fb/framebuffer.txt
pub struct Canvas {
    fb: Framebuffer,
    width: u32,
    height: u32,
    line_length: u32,
    frame: Vec<u8>,
    curr_color: (u8, u8, u8),
}

impl Canvas {
    pub fn new() -> Canvas {
        let fb = Framebuffer::new("/dev/fb0").expect("Failed to open fb0");
        let line_length = fb.fix_screen_info.line_length;
        let height = fb.var_screen_info.yres;
        Canvas {
            width: fb.var_screen_info.xres,
            height,
            frame: vec![0; (line_length * height) as usize],
            line_length,
            fb,
            curr_color: (255, 255, 255),
        }
    }

    /// Sets current drawing color.
    pub fn color(&mut self, r: u8, g: u8, b: u8) {
        self.curr_color = (r, g, b)
    }

    pub fn line(&mut self, from: Point, to: Point) {
        let (m, b) = math::solve_linear_eq(from, to);
        for x in from.0..to.0 {
            let y = (m * x as f64 + b) as u32;
            self.point(x, y);
        }
    }

    pub fn rect(&mut self, left_top: Point, right_bottom: Point) {
        for y in left_top.1..right_bottom.1 {
            for x in left_top.0..right_bottom.0 {
                self.point(x, y);
            }
        }
    }

    pub fn point(&mut self, x: u32, y: u32) {
        let start = (y * self.line_length + x * 4) as usize;
        self.frame[start]  = self.curr_color.2;
        self.frame[start + 1]  = self.curr_color.1;
        self.frame[start + 2]  = self.curr_color.0;
    }

    pub fn draw(&mut self) {
        self.fb.write_frame(&self.frame);
    }
}
