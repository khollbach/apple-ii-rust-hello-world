#![no_std]

mod mem;
mod screen;

use panic_halt as _;

use crate::screen::{Page, Point, Screen};

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    screen::text(false);
    screen::hires(false);
    screen::clear_lowres();

    let mut screen = Screen::new();

    let mut curr_page = Page::Page1;

    let initial_cells = [(19, 12), (20, 11), (20, 12), (20, 13), (21, 11)];
    for (x, y) in initial_cells {
        let p = Point::new(x, y);
        screen.draw(curr_page, p, 0xff);
    }

    loop {
        let (dx, dy) = screen.dims_xy();
        for y in 0..dy {
            for x in 0..dx {
                let p = Point::new(x, y);
                let alive = screen.read(curr_page, p) != 0;
                let n = count_live_neighbors(&screen, curr_page, p);
                let should_live = if alive { n == 2 || n == 3 } else { n == 3 };

                let pixel = if should_live { 0xff } else { 0 };
                screen.draw(curr_page.other(), p, pixel);
            }
        }

        curr_page = curr_page.other();
        screen.select_page(curr_page);
    }
}

fn count_live_neighbors(screen: &Screen, page: Page, p: Point) -> u8 {
    let mut count = 0;
    for p2 in p.neighbors() {
        if screen.read(page, p2) != 0 {
            count += 1;
        }
    }
    count
}
