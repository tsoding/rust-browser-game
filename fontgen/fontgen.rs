use std::os::raw::{c_char, c_uchar};
use std::ffi::CString;

type Pixel = u8;
const COMPS: i32 = 1;

#[link(name = "stb_image")]
extern "C" {
    fn stbi_load(filename: *const c_char,
                 x: *mut i32, y: *mut i32,
                 comp: *mut i32,
                 req_comp: i32) -> *mut c_uchar;
}

fn main() {
    const IMAGE_WIDTH: i32 = 128;
    const IMAGE_HEIGHT: i32 = 64;

    const FILE_PATH: &str = "./charmap-oldschool_white.png";
    let file_path_cstr = CString::new(FILE_PATH).expect("CString::new failed");

    let (mut w, mut h, mut n) = (0, 0, 0);
    let pixels: *mut Pixel = unsafe {
        stbi_load(file_path_cstr.into_raw(), &mut w, &mut h, &mut n, COMPS) as *mut Pixel
    };

    if pixels == std::ptr::null_mut() {
        panic!("Could not read file {}", FILE_PATH);
    }

    assert!(IMAGE_WIDTH == w);
    assert!(IMAGE_HEIGHT == h);

    const CHUNK_SIZE: i32 = 8;
    const CHUNK_COUNT: i32 = IMAGE_WIDTH * IMAGE_HEIGHT / CHUNK_SIZE;

    let mut chunks: [u8; CHUNK_COUNT as usize] = [0; CHUNK_COUNT as usize];

    for chunk_index in 0..CHUNK_COUNT {
        let chunk = &mut chunks[chunk_index as usize];
        for bit_index in 0..CHUNK_SIZE {
            let pixel = unsafe {
                *pixels.offset((chunk_index * CHUNK_SIZE + bit_index) as isize)
            };

            match pixel {
                0xFF => *chunk = (*chunk << 1) | 1,
                0x00 => *chunk = *chunk << 1,
                _ => panic!("Unknown pixel {:#02x}", *chunk),
            }
        }
    }

    const ROW_SIZE: usize = 16;
    const ROW_COUNT: usize = CHUNK_COUNT as usize / ROW_SIZE;
    println!("// Copy-paste this into your code");
    println!("// Generated from `{}`", FILE_PATH);
    println!("const COMPRESSED_FONT: [u8; {}] = [", CHUNK_COUNT);
    for row in 0..ROW_COUNT {
        print!("    ");
        for col in 0..ROW_SIZE {
            print!("{:#04x}, ", chunks[row * ROW_SIZE + col]);
        }
        println!("");
    }
    println!("];");
}