#![feature(start)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![no_std]
#![no_main]

mod boot;
mod terminal;
mod vga;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    terminal::initialize();
    terminal::write(b"HELLO KERNEL");

    loop {}
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            terminal::write(b"HELLO KERNEL");
        }
    }
}
