#![feature(start)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![no_std]

mod boot;

use core::panic::PanicInfo;

#[repr(u8)]
enum VgaColor {
    BLACK = 0,
    BLUE = 1,
    GREEN = 2,
    CYAN = 3,
    RED = 4,
    MAGENTA = 5,
    BROWN = 6,
    LIGHT_GREY = 7,
    DARK_GREY = 8,
    LIGHT_BLUE = 9,
    LIGHT_GREEN = 10,
    LIGHT_CYAN = 11,
    LIGHT_RED = 12,
    LIGHT_MAGENTA = 13,
    LIGHT_BROWN = 14,
    WHITE = 15,
}

const fn vga_entry_color(fg: VgaColor, bg: VgaColor) -> u8 {
    return fg as u8 | ((bg as u8) << 4);
}

const fn vga_entry(uc: u8, color: u8) -> u16 {
    let c: u16 = color as u16;

    return (uc as u16) | ((c as u16) << 8);
}

mod terminal {
    use super::vga_entry;
    use super::vga_entry_color;
    use super::VgaColor;

    const VGA_WIDTH: usize = 80;
    const VGA_HEIGHT: usize = 25;

    static mut row: usize = 0;
    static mut column: usize = 0;

    static mut color: u8 = vga_entry_color(VgaColor::LIGHT_GREY, VgaColor::BLACK);
    const VGA_PTR: *mut u16 = 0xB8000 as *mut u16;

    pub unsafe fn initialize() {
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                putCharAt(b' ', color, x, y);
            }
        }
    }

    unsafe fn putCharAt(c: u8, new_color: u8, x: usize, y: usize) {
        let index = y * VGA_WIDTH + x;

        unsafe { core::ptr::write_volatile(VGA_PTR, vga_entry(c, new_color)) }
    }

    unsafe fn putChar(c: u8) {
        putCharAt(c, color, column, row);
        column += 1;
        if (column == VGA_WIDTH) {
            column = 0;
            row += 1;
            if (row == VGA_HEIGHT) {
                row = 0;
            }
        }
    }

    pub unsafe fn write(slice: &[u8]) {
        for byte in slice.iter() {
            putChar(*byte);
        }
    }
}

#[export_name = "_start"]
#[no_mangle]
extern "C" fn _start() -> ! {
    unsafe {
        terminal::initialize();
        terminal::write(b"HELLO KERNEL");
    }

    loop {}
}

#[start]
fn main(_: isize, _: *const *const u8) -> isize {
    unsafe {
        terminal::initialize();
        terminal::write(b"HELLO KERNEL");
    }

    loop {}
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        unsafe {
            terminal::write(b"HELLO KERNEL");
        }
    }
}
