#[repr(u8)]
#[allow(unused)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

impl VgaColor {
    #[inline(always)]
    pub const fn vga_entry_color(self, bg: Self) -> Color {
        Color::new(self as u8 | ((bg as u8) << 4))
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Color {
    _color: u8,
}

const _: () = {
    use core::mem;
    assert!(mem::size_of::<Color>() == mem::size_of::<u8>());
    assert!(mem::align_of::<Color>() == mem::align_of::<u8>());
};

impl Color {
    #[inline(always)]
    pub(self) const fn new(color: u8) -> Self {
        Self { _color: color }
    }
}

use core::ptr;

//#[repr(packed)]
#[allow(unused)]
#[repr(C, align(2))]
struct Char {
    character: u8,
    color: Color,
}

const _: () = {
    use core::mem;
    assert!(mem::size_of::<Char>() == mem::size_of::<u16>());
    assert!(mem::align_of::<Char>() == mem::align_of::<u16>());
};

impl Char {
    #[inline(always)]
    pub(self) const fn new(character: u8, color: Color) -> Self {
        Self { character, color }
    }
}

const VGA_PTR: *mut Char = 0xB8000 as *mut Char;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

pub unsafe fn put_char_at(c: u8, color: Color, x: usize, y: usize) {
    let index = y * VGA_WIDTH + x;
    let offset = VGA_PTR.add(index);
    let element = Char::new(c, color);

    ptr::write_volatile(offset, element)
}
