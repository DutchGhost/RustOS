#![feature(start)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![no_std]

#[repr(packed)]
pub struct Multiboot {
    magic: i32,
    flags: i32,
    checksum: i32
}

const ALIGN: i32 = 1 << 0;
const MEMINFO: i32 = 1 << 1;
const MAGIC: i32 = 0x1BADB002;
const FLAGS: i32 = ALIGN | MEMINFO;

#[no_mangle]
#[link_section = ".multiboot"]
pub static MB: Multiboot = Multiboot {
    magic: MAGIC,
    flags: FLAGS,
    checksum: -(MAGIC + FLAGS),
};

/// 4 pages, PAGE_SIZE aligned.
#[repr(align(4096))]
pub struct AlignedStack([u8; 4096 * 4]);

/// The stack we start on.
///
/// The first thing we do is to make $esp point to it.
#[link_section = ".bss"]
pub static mut STACK: AlignedStack = AlignedStack([0; 4096 * 4]);

use core::panic::PanicInfo;

extern fn _start() -> ! {
    loop {}
}

#[start]
fn main (_: isize, _: *const *const u8) -> isize {
    0
}

#[lang = "eh_personality"]
extern fn rust_eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}