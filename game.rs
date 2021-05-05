#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[derive(Clone, Copy)]
#[repr(C)]
struct Pixel(u32);

impl Pixel {
    const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(((a as u32) << (3 * 8)) |
             ((b as u32) << (2 * 8)) |
             ((g as u32) << (1 * 8)) |
             ((r as u32) << (0 * 8)))
    }
}

const RED: Pixel   = Pixel::rgba(0xFF, 0, 0, 0xFF);
const GREEN: Pixel = Pixel::rgba(0, 0xFF, 0, 0xFF);
const BLUE: Pixel  = Pixel::rgba(0, 0, 0xFF, 0xFF);
const BACKGROUND: Pixel = Pixel::rgba(18, 18, 18, 0xFF);

pub struct Display {
    pixels: [Pixel; WIDTH * HEIGHT],
}

const fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

const fn min(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

const fn clamp(x: i32, low: i32, high: i32) -> i32 {
    min(max(low, x), high)
}

impl Display {
    fn fill(&mut self, pixel: Pixel) {
        unsafe {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    *self.pixels.get_unchecked_mut(y * WIDTH + x) = pixel;
                }
            }
        }
    }

    fn fill_rect(&mut self, x0: i32, y0: i32, w: i32, h: i32, pixel: Pixel) {
        let x1 = clamp(x0,         0, (WIDTH - 1)  as i32) as usize;
        let x2 = clamp(x0 + w - 1, 0, (WIDTH - 1)  as i32) as usize;
        let y1 = clamp(y0,         0, (HEIGHT - 1) as i32) as usize;
        let y2 = clamp(y0 + h - 1, 0, (HEIGHT - 1) as i32) as usize;

        unsafe {
            for y in y1..=y2 {
                for x in x1..=x2 {
                    *self.pixels.get_unchecked_mut(y * WIDTH + x) = pixel;
                }
            }
        }
    }
}

type Seconds = f32;

static mut DISPLAY: Display = Display {
    pixels: [Pixel(0); WIDTH * HEIGHT]
};

pub struct State {
    time: Seconds,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

const RECT_WIDTH: i32 = 100;
const RECT_HEIGHT: i32 = 100;

impl State {
    fn update(&mut self, dt: Seconds) {
        self.time += dt;

        const SPEED: i32 = 16;

        if self.x < 0 || self.x + RECT_WIDTH > WIDTH as i32 {
            self.dx = -self.dx;
        }

        if self.y < 0 || self.y + RECT_HEIGHT > HEIGHT as i32 {
            self.dy = -self.dy;
        }

        self.x += self.dx * SPEED;
        self.y += self.dy * SPEED;
    }

    fn render(&self, display: &mut Display) {
        display.fill(BACKGROUND);
        display.fill_rect(self.x, self.y, RECT_WIDTH, RECT_HEIGHT, RED);
    }
}

static mut STATE: State = State {
    time: 0.0,
    x: 10,
    y: 10,
    dx: 1,
    dy: 1,
};

#[no_mangle]
pub fn get_display_width() -> usize {
    WIDTH
}

#[no_mangle]
pub fn get_display_height() -> usize {
    HEIGHT
}

#[no_mangle]
pub unsafe fn get_display() -> &'static mut Display {
    &mut DISPLAY
}

#[no_mangle]
pub unsafe fn next_frame(dt: Seconds) {
    STATE.update(dt);
    STATE.render(&mut DISPLAY);
}

#[no_mangle]
pub fn move_right() {
}

#[no_mangle]
pub fn move_left() {
}

#[allow(dead_code)]
extern "C" {
    fn js_sin(x: f32) -> f32;
    fn js_cos(x: f32) -> f32;
}
