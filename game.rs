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

struct Player {
    x: i32,
    y: i32,
}

const PLAYER_WIDTH: i32 = 100;
const PLAYER_HEIGHT: i32 = 100;
const PLAYER_COLOR: Pixel = RED;
const PLAYER_SPEED: i32 = 16;

impl Player {
    fn render(&self, display: &mut Display) {
        display.fill_rect(
            self.x - PLAYER_WIDTH / 2,
            self.y - PLAYER_HEIGHT / 2,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            PLAYER_COLOR);
    }
}

pub struct State {
    time: Seconds,
    player: Player,
}

impl State {
    fn update(&mut self, dt: Seconds) {
        self.time += dt;
    }

    fn render(&self, display: &mut Display) {
        display.fill(BACKGROUND);
        self.player.render(display);
    }

    fn mouse_move(&mut self, x: i32, _y: i32) {
        self.player.x = x;
    }
}

static mut STATE: State = State {
    time: 0.0,
    player: Player{ x: 0, y: HEIGHT as i32 - PLAYER_HEIGHT },
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
pub unsafe fn mouse_move(x: i32, y: i32) {
    STATE.mouse_move(x, y);
}

#[allow(dead_code)]
extern "C" {
    fn js_sin(x: f32) -> f32;
    fn js_cos(x: f32) -> f32;
}
