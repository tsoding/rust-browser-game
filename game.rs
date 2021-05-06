#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

const DISPLAY_WIDTH: usize = 800;
const DISPLAY_HEIGHT: usize = 600;
const DISPLAY_BACKGROUND: Pixel = Pixel::rgba(0x3F, 0x3F, 0x3F, 0xFF);
const PLAYER_SIZE: i32 = 80;
const PLAYER_COLOR: Pixel = Pixel::rgba(0xDF, 0xAF, 0x8F, 0xFF);
const PLAYER_KILL_REWARD: usize = 100;
const BULLET_SIZE: i32 = 25;
const BULLET_SPEED: i32 = 20;
const BULLET_COLOR: Pixel = Pixel::rgba(0xEC, 0xB3, 0xB3, 0xFF);
const ENEMY_SIZE: i32 = 100;
const ENEMY_COLOR: Pixel = Pixel::rgba(0x7C, 0xB8, 0xBB, 0xFF);
const ENEMY_SPEED: i32 = 5;
const SCORE_LABEL_COLOR: Pixel = Pixel::rgba(0xDC, 0xDC, 0xCC, 0xFF);
const TEXT_SHADOW_COLOR: Pixel = Pixel::rgba(0, 0, 0, 0xFF);
const TEXT_SHADOW_OFFSET: i32 = 3;
const BULLETS_CAPACITY: usize = 5;
const ENEMIES_CAPACITY: usize = 10;
const ENEMY_SPAWN_PERIOD: Seconds = 1.0;
// Generated from `./charmap-oldschool_white.png`
const COMPRESSED_FONT: [u8; 622] = [
    0x00, 0x11, 0x20, 0xa1, 0x41, 0x0c, 0x0e, 0x08, 0x08, 0x40, 0x00, 0x05, 0x38, 0x20, 0x00, 0x01,
    0x20, 0xa1, 0x43, 0xcc, 0x92, 0x08, 0x10, 0x21, 0x50, 0x80, 0x00, 0x02, 0x02, 0x44, 0x60, 0x00,
    0x01, 0x20, 0x03, 0xe5, 0x01, 0x14, 0x00, 0x01, 0x20, 0x10, 0xe0, 0x80, 0x00, 0x02, 0x04, 0x4c,
    0xa0, 0x00, 0x01, 0x20, 0x01, 0x43, 0x82, 0x08, 0x00, 0x01, 0x20, 0x11, 0xf3, 0xe0, 0x0f, 0x80,
    0x08, 0x54, 0x20, 0x00, 0x01, 0x20, 0x03, 0xe1, 0x44, 0x15, 0x00, 0x01, 0x20, 0x10, 0xe0, 0x81,
    0x00, 0x02, 0x10, 0x64, 0x20, 0x00, 0x02, 0x01, 0x47, 0x89, 0x92, 0x00, 0x01, 0x10, 0x21, 0x50,
    0x81, 0x00, 0x02, 0x20, 0x44, 0x20, 0x00, 0x01, 0x20, 0x01, 0x41, 0x01, 0x8d, 0x00, 0x01, 0x08,
    0x40, 0x00, 0x01, 0x02, 0x00, 0x01, 0x04, 0x00, 0x01, 0x38, 0xf8, 0x00, 0x20, 0x38, 0x70, 0x63,
    0xe3, 0x8f, 0x8e, 0x1c, 0x00, 0x04, 0x07, 0x0e, 0x1c, 0x78, 0x70, 0x44, 0x88, 0xa2, 0x04, 0x00,
    0x01, 0x91, 0x22, 0x10, 0x20, 0x20, 0x02, 0x08, 0x91, 0x22, 0x44, 0x88, 0x04, 0x09, 0x22, 0x04,
    0x01, 0x11, 0x22, 0x00, 0x02, 0x43, 0xe1, 0x08, 0x97, 0x22, 0x44, 0x80, 0x08, 0x31, 0xf3, 0xc7,
    0x82, 0x0e, 0x1e, 0x00, 0x02, 0x80, 0x00, 0x01, 0x81, 0x15, 0x3e, 0x78, 0x80, 0x10, 0x08, 0x20,
    0x24, 0x44, 0x11, 0x02, 0x00, 0x01, 0x20, 0x43, 0xe1, 0x02, 0x17, 0x22, 0x44, 0x80, 0x20, 0x88,
    0x20, 0x24, 0x44, 0x11, 0x02, 0x10, 0x20, 0x20, 0x02, 0x00, 0x01, 0x10, 0x22, 0x44, 0x88, 0x7c,
    0x70, 0x23, 0xc3, 0x84, 0x0e, 0x1c, 0x00, 0x01, 0x40, 0x00, 0x02, 0x02, 0x0e, 0x22, 0x78, 0x70,
    0x00, 0x20, 0x78, 0xf9, 0xf1, 0xc4, 0x4f, 0x9f, 0x22, 0x40, 0x89, 0x11, 0xc7, 0x87, 0x1e, 0x1e,
    0x7c, 0x88, 0x44, 0x81, 0x02, 0x24, 0x42, 0x01, 0x22, 0x40, 0xd9, 0x12, 0x24, 0x48, 0x91, 0x20,
    0x10, 0x88, 0x44, 0x81, 0x02, 0x04, 0x42, 0x01, 0x24, 0x40, 0xa9, 0x92, 0x24, 0x48, 0x91, 0x20,
    0x10, 0x88, 0x44, 0xf1, 0xe2, 0x07, 0xc2, 0x01, 0x38, 0x40, 0x89, 0x52, 0x27, 0x88, 0x9e, 0x1c,
    0x10, 0x88, 0x44, 0x81, 0x02, 0x64, 0x42, 0x01, 0x24, 0x40, 0x89, 0x32, 0x24, 0x0a, 0x91, 0x02,
    0x10, 0x88, 0x44, 0x81, 0x02, 0x24, 0x42, 0x11, 0x22, 0x40, 0x89, 0x12, 0x24, 0x09, 0x11, 0x02,
    0x10, 0x88, 0x78, 0xf9, 0x01, 0xc4, 0x4f, 0x8e, 0x22, 0x7c, 0x89, 0x11, 0xc4, 0x06, 0x91, 0x3c,
    0x10, 0x70, 0x00, 0x20, 0x44, 0x89, 0x12, 0x27, 0xc3, 0x00, 0x01, 0x18, 0x10, 0x00, 0x01, 0x80,
    0x04, 0x00, 0x01, 0x01, 0x00, 0x01, 0x18, 0x00, 0x01, 0x44, 0x89, 0x12, 0x20, 0x42, 0x10, 0x08,
    0x28, 0x00, 0x01, 0x40, 0x04, 0x00, 0x01, 0x01, 0x00, 0x01, 0x20, 0x00, 0x01, 0x44, 0x88, 0xa1,
    0x40, 0x82, 0x08, 0x08, 0x00, 0x02, 0x01, 0xc7, 0x87, 0x0f, 0x1c, 0x7c, 0x78, 0x44, 0x88, 0x40,
    0x81, 0x02, 0x04, 0x08, 0x00, 0x03, 0x24, 0x48, 0x91, 0x22, 0x20, 0x88, 0x44, 0xa8, 0xa0, 0x82,
    0x02, 0x02, 0x08, 0x00, 0x02, 0x01, 0xe4, 0x48, 0x11, 0x3e, 0x20, 0x78, 0x28, 0xd9, 0x10, 0x84,
    0x02, 0x01, 0x08, 0x00, 0x02, 0x02, 0x24, 0x48, 0x91, 0x20, 0x20, 0x08, 0x10, 0x89, 0x10, 0x87,
    0xc3, 0x00, 0x01, 0x18, 0x00, 0x01, 0xf8, 0x01, 0xe7, 0x87, 0x0f, 0x1e, 0x20, 0x70, 0x00, 0x20,
    0x40, 0x20, 0x12, 0x04, 0x00, 0x06, 0x02, 0x00, 0x05, 0x40, 0x00, 0x01, 0x02, 0x04, 0x00, 0x06,
    0x02, 0x00, 0x05, 0x78, 0xe0, 0x72, 0x44, 0x0d, 0x1e, 0x1c, 0x78, 0x79, 0x61, 0xe7, 0x88, 0x91,
    0x22, 0x44, 0x88, 0x44, 0x20, 0x13, 0x84, 0x0a, 0x91, 0x22, 0x44, 0x89, 0x92, 0x02, 0x08, 0x91,
    0x22, 0x28, 0x88, 0x44, 0x20, 0x12, 0x44, 0x0a, 0x91, 0x22, 0x78, 0x79, 0x01, 0xc2, 0x08, 0x91,
    0x22, 0x10, 0x78, 0x44, 0x21, 0x12, 0x24, 0x08, 0x91, 0x22, 0x40, 0x09, 0x00, 0x01, 0x22, 0x48,
    0x8a, 0x2a, 0x28, 0x08, 0x44, 0xf8, 0xe2, 0x23, 0x88, 0x91, 0x1c, 0x40, 0x09, 0x03, 0xc1, 0x87,
    0x84, 0x14, 0x44, 0x70, 0x00, 0x21, 0x10, 0x41, 0x00, 0x0e, 0x20, 0x40, 0x80, 0x00, 0x0c, 0x7c,
    0x20, 0x40, 0x82, 0x40, 0x00, 0x0b, 0x08, 0x40, 0x40, 0x45, 0x80, 0x00, 0x0b, 0x10, 0x20, 0x40,
    0x80, 0x00, 0x0c, 0x20, 0x20, 0x40, 0x80, 0x00, 0x0c, 0x7c, 0x10, 0x41, 0x00, 0xbd,
];

#[derive(Clone, Copy)]
#[repr(C)]
struct Pixel(u32);

impl Pixel {
    const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(
            ((a as u32) << (3 * 8))
                | ((b as u32) << (2 * 8))
                | ((g as u32) << (1 * 8))
                | ((r as u32) << (0 * 8)),
        )
    }
}

#[repr(C)]
pub struct Display {
    pixels: [Pixel; DISPLAY_WIDTH * DISPLAY_HEIGHT],
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
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                unsafe {
                    *self.pixels.get_unchecked_mut(y * DISPLAY_WIDTH + x) = pixel;
                }
            }
        }
    }

    fn fill_rect(&mut self, x0: i32, y0: i32, w: i32, h: i32, pixel: Pixel) {
        let x1 = clamp(x0, 0, (DISPLAY_WIDTH - 1) as i32) as usize;
        let x2 = clamp(x0 + w - 1, 0, (DISPLAY_WIDTH - 1) as i32) as usize;
        let y1 = clamp(y0, 0, (DISPLAY_HEIGHT - 1) as i32) as usize;
        let y2 = clamp(y0 + h - 1, 0, (DISPLAY_HEIGHT - 1) as i32) as usize;

        unsafe {
            for y in y1..=y2 {
                for x in x1..=x2 {
                    *self.pixels.get_unchecked_mut(y * DISPLAY_WIDTH + x) = pixel;
                }
            }
        }
    }

    fn put(&mut self, x: i32, y: i32, pixel: Pixel) {
        if 0 <= x && x < DISPLAY_WIDTH as i32 && 0 <= y && y < DISPLAY_HEIGHT as i32 {
            unsafe {
                *self.pixels.get_unchecked_mut(y as usize * DISPLAY_WIDTH + x as usize) = pixel;
            }
        }
    }
}

type Seconds = f32;

#[derive(Copy, Clone)]
#[repr(C)]
struct Entity {
    x: i32,
    y: i32,
    alive: bool,
}

impl Entity {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y, alive: true }
    }

    const fn dead() -> Self {
        Self {
            x: 0,
            y: 0,
            alive: false,
        }
    }

    fn revive(&mut self, x: i32, y: i32) {
        self.alive = true;
        self.x = x;
        self.y = y;
    }

    fn render(&self, display: &mut Display, size: i32, color: Pixel) {
        if self.alive {
            display.fill_rect(self.x - size / 2, self.y - size / 2, size, size, color);
        }
    }

    fn overlaps(&self, self_size: i32, that: &Self, that_size: i32) -> bool {
        let left1 = self.x - self_size / 2;
        let right1 = self.x + self_size / 2;
        let top1 = self.y - self_size / 2;
        let bottom1 = self.y + self_size / 2;

        let left2 = that.x - that_size / 2;
        let right2 = that.x + that_size / 2;
        let top2 = that.y - that_size / 2;
        let bottom2 = that.y + that_size / 2;

        right1 >= left2 && right2 >= left1 && bottom2 >= top1 && bottom1 >= top2
    }
}

const FONT_IMAGE_WIDTH: usize = 128;
const FONT_IMAGE_HEIGHT: usize = 64;
const FONT_IMAGE_COLS: usize = 18;
const FONT_IMAGE_ROWS: usize = 7;
const FONT_CHAR_WIDTH: usize = FONT_IMAGE_WIDTH / FONT_IMAGE_COLS;
const FONT_CHAR_HEIGHT: usize = FONT_IMAGE_HEIGHT / FONT_IMAGE_ROWS;

const CHUNK_SIZE: usize = 8;

struct Font {
    pixels: [u8; FONT_IMAGE_WIDTH * FONT_IMAGE_HEIGHT],
}

impl Font {
    fn decompress_from_bytes(&mut self, bytes: &[u8]) {
        let n = bytes.len();
        let mut i = 0;
        let mut pixels_size = 0;
        while i < n {
            let byte = unsafe {bytes.get_unchecked(i)};

            if bytes[i] == 0x00 {
                i += 1;
                pixels_size += unsafe {*bytes.get_unchecked(i) as usize} * 8;
                i += 1;
            } else {
                for bit_index in 0..CHUNK_SIZE {
                    unsafe {
                        *self.pixels.get_unchecked_mut(pixels_size) =
                            ((byte >> (CHUNK_SIZE - bit_index - 1)) & 1) * 0xFF;
                    }
                    pixels_size += 1;
                }
                i += 1;
            }
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&u8> {
        if 0 <= x && x < FONT_IMAGE_WIDTH as i32 && 0 <= y && y < FONT_IMAGE_HEIGHT as i32 {
            unsafe {
                Some(self.pixels.get_unchecked(y as usize * FONT_IMAGE_WIDTH + x as usize))
            }
        } else {
            None
        }
    }

    fn render_ascii(&self,
                    display: &mut Display,
                    code: u8,
                    start_x: i32, start_y: i32,
                    scale: i32,
                    color: Pixel) {
        if 32 <= code && code <= 126 {
            let char_x = (code - 32) as usize % FONT_IMAGE_COLS;
            let char_y = (code - 32) as usize / FONT_IMAGE_COLS;

            for y in 0..FONT_CHAR_HEIGHT as i32 {
                for x in 0..FONT_CHAR_WIDTH as i32 {
                    for scale_x in 0..scale {
                        for scale_y in 0..scale {
                            let font_x = char_x as i32 * FONT_CHAR_WIDTH as i32 + x;
                            let font_y = char_y as i32 * FONT_CHAR_HEIGHT as i32 + y;
                            let display_x = start_x + x * scale + scale_x;
                            let display_y = start_y + y * scale + scale_y;

                            if let Some(alpha) = self.get(font_x, font_y) {
                                if *alpha == 0xFF {
                                    display.put(display_x, display_y, color);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self.render_ascii(display, '?' as u8, start_x, start_y, scale, color)
        }
    }
}

const LABEL_CAPACITY: usize = 64;
struct Label {
    chars: [u8; LABEL_CAPACITY],
    count: usize,
}

impl Label {
    const fn empty() -> Self {
        Self {
            chars: [0; LABEL_CAPACITY],
            count: 0,
        }
    }

    fn render(&self,
              display: &mut Display,
              font: &Font,
              x: i32, y: i32,
              scale: i32,
              color: Pixel) {
        if self.count < LABEL_CAPACITY {
            for i in 0..self.count {
                font.render_ascii(
                    display,
                    unsafe { *self.chars.get_unchecked(i) },
                    x + i as i32 * FONT_CHAR_WIDTH as i32 * scale, y,
                    scale,
                    color);
            }
        }
    }

    fn clear(&mut self) {
        self.count = 0;
    }

    fn push_byte(&mut self, b: u8) {
        if self.count < LABEL_CAPACITY {
            unsafe {
                *self.chars.get_unchecked_mut(self.count) = b;
            }
            self.count += 1;
        }
    }

    fn push_bytes(&mut self, bs: &[u8]) {
        for b in bs {
            self.push_byte(*b);
        }
    }

    fn push_usize(&mut self, mut x: usize) {
        let saved_count = self.count;

        if x == 0 {
            self.push_byte(b'0');
        } else {
            while x > 0 && self.count < LABEL_CAPACITY {
                self.push_byte((x % 10) as u8 + b'0');
                x /= 10;
            }

            if x > 0 {
                // x does not fit into the Label rolling back and quitting
                self.count = saved_count;
                return;
            }
        }

        let mut a = saved_count;
        let mut b = self.count - 1;

        while a < b {
            unsafe {
                let t = *self.chars.get_unchecked_mut(a);
                *self.chars.get_unchecked_mut(a) =
                    *self.chars.get_unchecked_mut(b);
                *self.chars.get_unchecked_mut(b) = t;
            }

            a += 1;
            b -= 1;
        }
    }
}

#[repr(C)]
pub struct State {
    player: Entity,
    bullets: [Entity; BULLETS_CAPACITY],
    enemies: [Entity; ENEMIES_CAPACITY],
    enemy_spawn_cooldown: Seconds,
    pause: bool,
    score: usize,
    label: Label,
}

impl State {
    const fn default() -> Self {
        Self {
            player: Entity::new(0, DISPLAY_HEIGHT as i32 - PLAYER_SIZE),
            bullets: [Entity::dead(); BULLETS_CAPACITY],
            enemies: [Entity::dead(); ENEMIES_CAPACITY],
            enemy_spawn_cooldown: 0.0,
            pause: false,
            score: 0,
            label: Label::empty(),
        }
    }

    fn update(&mut self, dt: Seconds) {
        if !self.pause {
            for bullet in self.bullets.iter_mut() {
                if bullet.alive {
                    bullet.y -= BULLET_SPEED;
                    if bullet.y < 0 {
                        bullet.alive = false
                    }
                }
            }
            for enemy in self.enemies.iter_mut() {
                if enemy.alive {
                    enemy.y += ENEMY_SPEED;
                    if enemy.y > DISPLAY_HEIGHT as i32 {
                        enemy.alive = false
                    }
                }
            }

            for enemy in self.enemies.iter_mut() {
                if enemy.alive {
                    for bullet in self.bullets.iter_mut() {
                        if bullet.alive {
                            if enemy.overlaps(ENEMY_SIZE, bullet, BULLET_SIZE) {
                                enemy.alive = false;
                                bullet.alive = false;
                                self.score += PLAYER_KILL_REWARD;
                                break;
                            }
                        }
                    }
                }

                // Enemy could be killed by the bullet in the previous
                // condition. So we need to check if it's alive again.
                if enemy.alive {
                    if enemy.overlaps(ENEMY_SIZE, &self.player, PLAYER_SIZE) {
                        self.player.alive = false;
                    }
                }
            }

            self.enemy_spawn_cooldown -= dt;
            if self.enemy_spawn_cooldown < 0.0 {
                self.spawn_enemy(self.player.x, 0);
                self.enemy_spawn_cooldown = ENEMY_SPAWN_PERIOD;
            }

            self.label.clear();
            self.label.push_bytes(b"Score: ");
            self.label.push_usize(self.score);
        }
    }

    fn render(&self, display: &mut Display, font: &Font) {
        if !self.pause {
            display.fill(DISPLAY_BACKGROUND);
            self.player.render(display, PLAYER_SIZE, PLAYER_COLOR);
            for bullet in self.bullets.iter() {
                bullet.render(display, BULLET_SIZE, BULLET_COLOR)
            }
            for enemy in self.enemies.iter() {
                enemy.render(display, ENEMY_SIZE, ENEMY_COLOR)
            }

            self.label.render(display, font,
                              0, 0, 4,
                              TEXT_SHADOW_COLOR);
            self.label.render(display, font,
                              TEXT_SHADOW_OFFSET, TEXT_SHADOW_OFFSET, 4,
                              SCORE_LABEL_COLOR);
        }
    }

    fn spawn_enemy(&mut self, x: i32, y: i32) {
        for enemy in self.enemies.iter_mut() {
            if !enemy.alive {
                enemy.revive(x, y);
                break;
            }
        }
    }

    fn spawn_bullet(&mut self, x: i32, y: i32) {
        for bullet in self.bullets.iter_mut() {
            if !bullet.alive {
                bullet.revive(x, y);
                break;
            }
        }
    }

    fn mouse_move(&mut self, x: i32, _y: i32) {
        if self.player.alive {
            self.player.x = x;
        }
    }

    fn mouse_click(&mut self) {
        if self.player.alive {
            self.spawn_bullet(
                self.player.x,
                self.player.y - PLAYER_SIZE / 2 - BULLET_SIZE / 2,
            );
        }
    }

    fn toggle_pause(&mut self) {
        self.pause = !self.pause
    }
}

static mut FONT: Font = Font {
    pixels: [0; 1024 * 8],
};
static mut STATE: State = State::default();
static mut DISPLAY: Display = Display {
    pixels: [Pixel(0); DISPLAY_WIDTH * DISPLAY_HEIGHT],
};


#[no_mangle]
pub unsafe fn init() {
    FONT.decompress_from_bytes(&COMPRESSED_FONT);
}

#[no_mangle]
pub fn get_display_width() -> usize {
    DISPLAY_WIDTH
}

#[no_mangle]
pub fn get_display_height() -> usize {
    DISPLAY_HEIGHT
}

#[no_mangle]
pub unsafe fn get_display() -> &'static mut Display {
    &mut DISPLAY
}

#[no_mangle]
pub unsafe fn next_frame(dt: Seconds) {
    STATE.update(dt);
    STATE.render(&mut DISPLAY, &FONT);
}

#[no_mangle]
pub unsafe fn mouse_move(x: i32, y: i32) {
    STATE.mouse_move(x, y);
}

#[no_mangle]
pub unsafe fn mouse_click() {
    STATE.mouse_click();
}

#[no_mangle]
pub unsafe fn toggle_pause() {
    STATE.toggle_pause();
}

#[allow(dead_code)]
extern "C" {
    fn js_sin(x: f32) -> f32;
    fn js_cos(x: f32) -> f32;
}
