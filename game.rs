#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

pub type Pixel = u32;

pub struct Display {
    pixels: [Pixel; WIDTH * HEIGHT],
}

static mut display: Display = Display {
    pixels: [0; WIDTH * HEIGHT]
};

pub struct State {
    
}

static mut state: State = State {};

#[no_mangle]
pub fn get_display() -> *mut Display {
    unsafe {
        &mut display
    }
}

#[no_mangle]
pub fn next_frame(dt: f32) -> i32 {
    unsafe {
        imported_func(69)
    }
}

extern "C" {
    fn imported_func(x: i32) -> i32;
}
