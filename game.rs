#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

const DISPLAY_WIDTH: usize        = 800;
const DISPLAY_HEIGHT: usize       = 600;
const DISPLAY_BACKGROUND: Pixel   = Pixel::rgba(0x3F, 0x3F, 0x3F, 0xFF);
const PLAYER_SIZE: i32            = 80;
const PLAYER_COLOR: Pixel         = Pixel::rgba(0xDF, 0xAF, 0x8F, 0xFF);
const PLAYER_KILL_REWARD: usize   = 100;
const BULLET_SIZE: i32            = 25;
const BULLET_SPEED: i32           = 20;
const BULLET_COLOR: Pixel         = Pixel::rgba(0xEC, 0xB3, 0xB3, 0xFF);
const ENEMY_SIZE: i32             = 100;
const ENEMY_COLOR: Pixel          = Pixel::rgba(0x7C, 0xB8, 0xBB, 0xFF);
const ENEMY_SPEED: i32            = 5;
const BULLETS_CAPACITY: usize     = 5;
const ENEMIES_CAPACITY: usize     = 10;
const ENEMY_SPAWN_PERIOD: Seconds = 1.0;

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
        unsafe {
            for y in 0..DISPLAY_HEIGHT {
                for x in 0..DISPLAY_WIDTH {
                    *self.pixels.get_unchecked_mut(y * DISPLAY_WIDTH + x) = pixel;
                }
            }
        }
    }

    fn fill_rect(&mut self, x0: i32, y0: i32, w: i32, h: i32, pixel: Pixel) {
        let x1 = clamp(x0,         0, (DISPLAY_WIDTH - 1)  as i32) as usize;
        let x2 = clamp(x0 + w - 1, 0, (DISPLAY_WIDTH - 1)  as i32) as usize;
        let y1 = clamp(y0,         0, (DISPLAY_HEIGHT - 1) as i32) as usize;
        let y2 = clamp(y0 + h - 1, 0, (DISPLAY_HEIGHT - 1) as i32) as usize;

        unsafe {
            for y in y1..=y2 {
                for x in x1..=x2 {
                    *self.pixels.get_unchecked_mut(y * DISPLAY_WIDTH + x) = pixel;
                }
            }
        }
    }
}

type Seconds = f32;

static mut DISPLAY: Display = Display {
    pixels: [Pixel(0); DISPLAY_WIDTH * DISPLAY_HEIGHT]
};

struct Player {
    x: i32,
    y: i32,
}

impl Player {
    fn render(&self, display: &mut Display) {
        display.fill_rect(
            self.x - PLAYER_SIZE / 2,
            self.y - PLAYER_SIZE / 2,
            PLAYER_SIZE,
            PLAYER_SIZE,
            PLAYER_COLOR);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Bullet {
    x: i32,
    y: i32,
    alive: bool,
}

impl Bullet {
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

    fn render(&self, display: &mut Display) {
        if self.alive {
            display.fill_rect(
                self.x - BULLET_SIZE / 2,
                self.y - BULLET_SIZE / 2,
                BULLET_SIZE,
                BULLET_SIZE,
                BULLET_COLOR);
        }
    }
}

#[derive(Copy, Clone)]
struct Enemy {
    x: i32,
    y: i32,
    alive: bool,
}

impl Enemy {
    const fn dead() -> Self {
        Self {
            x: 0,
            y: 0,
            alive: false,
        }
    }

    fn render(&self, display: &mut Display) {
        if self.alive {
            display.fill_rect(
                self.x - ENEMY_SIZE / 2,
                self.y - ENEMY_SIZE / 2,
                ENEMY_SIZE,
                ENEMY_SIZE,
                ENEMY_COLOR);
        }
    }

    fn revive(&mut self, x: i32, y: i32) {
        self.alive = true;
        self.x = x;
        self.y = y;
    }

    fn overlap_bullet(&self, bullet: &Bullet) -> bool {
        self.x - ENEMY_SIZE / 2 <= bullet.x &&
            bullet.x <= self.x + ENEMY_SIZE / 2 &&
            self.y - ENEMY_SIZE / 2 <= bullet.y &&
            bullet.y <= self.y + ENEMY_SIZE / 2
    }
}

pub struct State {
    player: Player,
    bullets: [Bullet; BULLETS_CAPACITY],
    enemies: [Enemy; ENEMIES_CAPACITY],
    enemy_spawn_cooldown: Seconds,
    pause: bool,
    score: usize,
}

impl State {
    const fn default() -> Self{
        Self {
            player: Player{ x: 0, y: DISPLAY_HEIGHT as i32 - PLAYER_SIZE },
            bullets: [Bullet::dead(); BULLETS_CAPACITY],
            enemies: [Enemy::dead(); ENEMIES_CAPACITY],
            enemy_spawn_cooldown: 0.0,
            pause: false,
            score: 0,
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
                            if enemy.overlap_bullet(&bullet) {
                                enemy.alive = false;
                                bullet.alive = false;
                                self.score += PLAYER_KILL_REWARD;
                                break;
                            }
                        }
                    }
                }
            }

            self.enemy_spawn_cooldown -= dt;
            if self.enemy_spawn_cooldown < 0.0 {
                self.spawn_enemy(self.player.x, 0);
                self.enemy_spawn_cooldown = ENEMY_SPAWN_PERIOD;
            }
        }
    }

    fn render(&self, display: &mut Display) {
        if !self.pause {
            display.fill(DISPLAY_BACKGROUND);
            self.player.render(display);
            for bullet in self.bullets.iter() {
                bullet.render(display)
            }
            for enemy in self.enemies.iter() {
                enemy.render(display)
            }
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
        self.player.x = x;
    }

    fn mouse_click(&mut self) {
        self.spawn_bullet(
            self.player.x,
            self.player.y - PLAYER_SIZE / 2 - BULLET_SIZE / 2);
    }

    fn toggle_pause(&mut self) {
        self.pause = !self.pause
    }
}

static mut STATE: State = State::default();

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
    STATE.render(&mut DISPLAY);
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
