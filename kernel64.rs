// Rust on BareMetal - Tested with Rust 0.13.0-nightly
// rustc -O --crate-type lib -o kernel64.o --emit obj kernel64.rs
// ld -T app.ld -o kernel64.sys kernel64.o

#![no_std]
#![allow(ctypes)]
#![allow(dead_code)]
#![feature(lang_items)]
#[lang="sized"]
trait Sized {}

enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yello       = 14,
    White       = 15,
}

enum Option<T> {
    None,
    Some(T)
}

struct IntRange {
    cur: int,
    max: int
}

impl IntRange {
    fn next(&mut self) -> Option<int> {
        if self.cur < self.max {
            self.cur += 1;
            Some(self.cur - 1)
        } else {
            None
        }
    }
}

fn range(lo: int, hi: int) -> IntRange {
    IntRange { cur: lo, max: hi }
}

fn clear_screen(background: Color) {
    let mut r = range(0, 80 * 25);
    loop {
        match r.next() {
            Some(x) => {
                unsafe {
                   *((0xb8000 + x * 2) as *mut u16) = (background as u16) << 12;
                }
            },
            None => {break}
        }
    }
}

#[no_mangle]
#[no_stack_check]
pub fn main() {
    clear_screen(LightBlue);
    loop {
		// Loop forever
    }
}
