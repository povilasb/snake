/// Graphics utils

use std::ptr;

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

    /// Draws vertical line.
    /// from and to x values must match.
    pub fn vline(&mut self, from: Point, to: Point) {
        for y in from.1..to.1 {
            self.point(from.0, y);
        }
    }

    /// Draws a filled rect.
    pub fn rect(&mut self, left_top: Point, right_bottom: Point) {
        for y in left_top.1..right_bottom.1 {
            for x in left_top.0..right_bottom.0 {
                self.point(x, y);
            }
        }
    }

    pub fn point(&mut self, x: u32, y: u32) {
        let start = (y * self.line_length + x * 4) as usize;
        self.frame[start] = self.curr_color.2;
        self.frame[start + 1] = self.curr_color.1;
        self.frame[start + 2] = self.curr_color.0;
    }

    /// Draw sprite to the frame.
    pub fn sprite_to(&mut self, x: u32, y: u32, img: &Sprite) {
        let mut start = (y * self.line_length + x * 4) as usize;
        for line in &img.data {
            self.frame[start..start + line.len()].copy_from_slice(&line);
            start += self.line_length as usize;
        }
    }

    pub fn draw(&mut self) {
        self.fb.write_frame(&self.frame);
    }

    pub fn clear(&mut self) {
        unsafe {
            ptr::write_bytes(
                self.frame.as_mut_ptr(),
                0,
                (self.line_length * self.height) as usize,
            );
        }
    }
}

pub struct Sprite {
    data: Vec<Vec<u8>>,
    width: usize,
}

impl Sprite {
    pub fn cell(color: (u8, u8, u8)) -> Sprite {
        let mut data = Vec::<Vec<u8>>::new();
        let width = 25;
        for _ in 0..25 {
            let mut line = Vec::<u8>::new();
            for _ in 0..width {
                line.push(color.0);
                line.push(color.1);
                line.push(color.2);
                line.push(0);
            }
            data.push(line);
        }

        Sprite { data, width }
    }

    pub fn head_left() -> Sprite {
        let mut sprite = Sprite::cell((0, 255, 255));
        let rgb = (0, 0, 255);
        for y in 5..9 {
            sprite.point(20, y, &rgb);
            sprite.point(24, y, &rgb);
            sprite.point(28, y, &rgb);
            sprite.point(32, y, &rgb);
        }
        for y in 17..21 {
            sprite.point(20, y, &rgb);
            sprite.point(24, y, &rgb);
            sprite.point(28, y, &rgb);
            sprite.point(32, y, &rgb);
        }
        sprite
    }

    pub fn head_right() -> Sprite {
        let mut sprite = Sprite::cell((0, 255, 255));
        let rgb = (0, 0, 255);
        let left_off = (sprite.width - sprite.width / 5 - 4) * 4;
        for y in 5..9 {
            sprite.point(left_off, y, &rgb);
            sprite.point(left_off + 4, y, &rgb);
            sprite.point(left_off + 8, y, &rgb);
            sprite.point(left_off + 12, y, &rgb);
        }
        for y in 17..21 {
            sprite.point(left_off, y, &rgb);
            sprite.point(left_off + 4, y, &rgb);
            sprite.point(left_off + 8, y, &rgb);
            sprite.point(left_off + 12, y, &rgb);
        }
        sprite
    }

    pub fn head_up() -> Sprite {
        let mut sprite = Sprite::cell((0, 255, 255));
        let rgb = (0, 0, 255);
        let xoff = 20;
        let xoff2 = (sprite.width - sprite.width / 5 - 4) * 4;
        for y in 5..9 {
            sprite.point(xoff, y, &rgb);
            sprite.point(xoff + 4, y, &rgb);
            sprite.point(xoff + 8, y, &rgb);
            sprite.point(xoff + 12, y, &rgb);
            sprite.point(xoff2, y, &rgb);
            sprite.point(xoff2 + 4, y, &rgb);
            sprite.point(xoff2 + 8, y, &rgb);
            sprite.point(xoff2 + 12, y, &rgb);
        }
        sprite
    }

    pub fn head_down() -> Sprite {
        let mut sprite = Sprite::cell((0, 255, 255));
        let rgb = (0, 0, 255);
        let xoff = 20;
        let xoff2 = (sprite.width - sprite.width / 5 - 4) * 4;
        for y in sprite.width - 8..sprite.width - 4 {
            sprite.point(xoff, y, &rgb);
            sprite.point(xoff + 4, y, &rgb);
            sprite.point(xoff + 8, y, &rgb);
            sprite.point(xoff + 12, y, &rgb);
            sprite.point(xoff2, y, &rgb);
            sprite.point(xoff2 + 4, y, &rgb);
            sprite.point(xoff2 + 8, y, &rgb);
            sprite.point(xoff2 + 12, y, &rgb);
        }
        sprite
    }

    pub fn body() -> Sprite {
        Sprite::cell((0, 255, 0))
    }

    fn point(&mut self, x: usize, y: usize, color: &(u8, u8, u8)) {
        self.data[y][x] = color.2;
        self.data[y][x + 1] = color.1;
        self.data[y][x + 2] = color.0;
        self.data[y][x + 3] = 0;
    }
}
