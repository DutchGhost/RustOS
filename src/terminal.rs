use crate::vga::{put_char_at, Color, VgaColor, VGA_HEIGHT, VGA_WIDTH};

static mut ROW: usize = 0;
static mut COLUMN: usize = 0;
static mut COLOR: Color = VgaColor::LightGreen.vga_entry_color(VgaColor::Black);

pub unsafe fn initialize() {
    for y in 0..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            put_char_at(b' ', COLOR, x, y);
        }
    }
}

unsafe fn put_char(c: u8) {
    put_char_at(c, COLOR, COLUMN, ROW);
    COLUMN += 1;
    if COLUMN == VGA_WIDTH {
        COLUMN = 0;
        ROW += 1;
        if ROW == VGA_HEIGHT {
            ROW = 0;
        }
    }
}

pub unsafe fn write(slice: &[u8]) {
    for byte in slice.iter() {
        put_char(*byte);
    }
}
