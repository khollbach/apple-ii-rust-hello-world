#![no_std]

mod mem;
mod screen;

use panic_halt as _;

use crate::screen::Point;

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    screen::text(false);
    screen::hires(false);
    screen::clear_lowres();

    let initial_cells = [(19, 12), (20, 11), (20, 12), (20, 13), (21, 11)];
    for (x, y) in initial_cells {
        Point::new(x, y).write(0xff);
    }

    loop {}
}
