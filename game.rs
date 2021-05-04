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

#[derive(Copy, Clone)]
struct RGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl RGBA {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {r, g, b, a}
    }

    fn to_pixel(self) -> Pixel {
        Pixel::rgba(
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8)
    }
}

pub struct Display {
    pixels: [[Pixel; WIDTH]; HEIGHT],
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
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.pixels[y][x] = pixel;
            }
        }
    }

    fn fill_rect(&mut self, x0: i32, y0: i32, w: i32, h: i32, pixel: Pixel) {
        // TODO: something here blows up the size of the final executable
        let x1 = clamp(x0,         0, (WIDTH - 1)  as i32) as usize;
        let x2 = clamp(x0 + w - 1, 0, (WIDTH - 1)  as i32) as usize;
        let y1 = clamp(y0,         0, (HEIGHT - 1) as i32) as usize;
        let y2 = clamp(y0 + h - 1, 0, (HEIGHT - 1) as i32) as usize;

        for y in y1..=y2 {
            for x in x1..=x2 {
                self.pixels[y][x] = pixel;
            }
        }
    }
}

type Seconds = f32;

static mut DISPLAY: Display = Display {
    pixels: [[Pixel(0); WIDTH]; HEIGHT]
};

pub struct State {
    time: Seconds,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl State {
    fn update(&mut self, dt: Seconds) {
        self.time += dt;
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
pub fn get_display() -> *mut Display {
    unsafe {
        &mut DISPLAY
    }
}

#[no_mangle]
pub fn next_frame(dt: Seconds) {
    unsafe {
        STATE.update(dt);
        DISPLAY.fill(BACKGROUND);

        const SPEED: i32 = 16;
        const RECT_WIDTH: i32 = 100;
        const RECT_HEIGHT: i32 = 100;

        if STATE.x < 0 || STATE.x + RECT_WIDTH > WIDTH as i32 {
            STATE.dx = -STATE.dx;
        }

        if STATE.y < 0 || STATE.y + RECT_HEIGHT > HEIGHT as i32 {
            STATE.dy = -STATE.dy;
        }

        STATE.x += STATE.dx * SPEED;
        STATE.y += STATE.dy * SPEED;
        DISPLAY.fill_rect(STATE.x, STATE.y, RECT_WIDTH, RECT_HEIGHT, RED);
    }
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
