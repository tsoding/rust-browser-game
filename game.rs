#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

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

impl Display {
    fn fill(&mut self, pixel: Pixel) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
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
}

impl State {
    fn update(&mut self, dt: Seconds) {
        self.time += dt;
    }
}

static mut STATE: State = State {
    time: 0.0
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
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let ix = x as f32 / WIDTH as f32;
                let iy = y as f32 / HEIGHT as f32;
                DISPLAY.pixels[y][x] =
                    RGBA::new(
                        (js_sin(STATE.time + ix) + 1.0) * 0.5,
                        (js_cos(STATE.time + iy) + 1.0) * 0.5,
                        (js_cos(STATE.time + iy + ix) + 1.0) * 0.5,
                        1.0)
                    .to_pixel();
            }
        }
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
