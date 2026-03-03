use crate::mem;

// todo: maybe there's a clean way to wrap up the soft-switches
// in a safe API?

// (low-res, for now)
pub struct Screen;

impl Screen {
    pub fn new() -> Self {
        Self
    }

    pub fn dims_xy(&self) -> (u8, u8) {
        (40, 24)
    }

    pub fn curr_page(&self) -> Page {
        let byte = unsafe { mem::read(0xC01C) };
        let flag = byte & 0x8 != 0;
        if flag { Page::Page2 } else { Page::Page1 }
    }

    pub fn select_page(&mut self, page: Page) {
        match page {
            Page::Page1 => unsafe { mem::read(0xC054) },
            Page::Page2 => unsafe { mem::read(0xC055) },
        };
    }

    pub fn read(&self, page: Page, point: Point) -> u8 {
        let mut addr = point.addr();
        if page == Page::Page2 {
            addr = unsafe { addr.offset(0x400) };
        }

        unsafe { addr.read_volatile() }
    }

    pub fn draw(&mut self, page: Page, point: Point, value: u8) {
        let mut addr = point.addr();
        if page == Page::Page2 {
            addr = unsafe { addr.offset(0x400) };
        }

        unsafe { addr.write_volatile(value) };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Page1,
    Page2,
}

impl Page {
    pub fn other(self) -> Self {
        match self {
            Page::Page1 => Page::Page2,
            Page::Page2 => Page::Page1,
        }
    }
}

/// Enable text mode.
pub fn text(enable: bool) {
    if enable {
        unsafe { mem::read(0xC051) };
    } else {
        unsafe { mem::read(0xC050) };
    }
}

/// Enable mixed graphics and text.
pub fn mixed(enable: bool) {
    if enable {
        unsafe { mem::read(0xC053) };
    } else {
        unsafe { mem::read(0xC052) };
    }
}

/// Enable high-resolution graphics.
pub fn hires(enable: bool) {
    if enable {
        unsafe { mem::read(0xC057) };
    } else {
        unsafe { mem::read(0xC056) };
    }
}

/// Write a black pixel to all low-res graphics memory.
/// 
/// (both pages)
pub fn clear_lowres() {
    for i in 0x400..0xc00 {
        unsafe { mem::write(i, 0) }
    }
}

/// A coordinate on the low-res (or text mode) screen.
///
/// Note that there's technically a "top-half" and a "bottom-half" to each of
/// these "pixels", in GR mode. Each can hold a 4-bit color.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        assert!(x < 40);
        assert!(y < 24);
        Self { x, y }
    }

    pub fn x(self) -> u8 {
        self.x
    }

    pub fn y(self) -> u8 {
        self.y
    }

    /// What is the memory address for this low-res pixel?
    fn addr(self) -> *mut u8 {
        let x: u16 = self.x.into();
        let y: u16 = self.y.into();

        let group = y / 8;
        let base = match group {
            0 => 0x400,
            1 => 0x428,
            2 => 0x450,
            _ => unreachable!(),
        };

        let offset = y % 8 * 0x80;

        let addr: u16 = base + offset + x;
        addr as _
    }
    
    pub fn neighbors(self) -> impl Iterator<Item=Point> {
        let x = self.x as i8;
        let y = self.y as i8;

        [-1, 0, 1].into_iter().flat_map(move |dy| {
            [-1, 0, 1].into_iter().filter_map(move |dx| {
                if (dx, dy) == (0, 0) {
                    return None;
                }

                let x2 = x + dx;
                let y2 = y + dy;

                if !in_bounds(x2, y2) {
                    return None;
                }
                Some(Point::new(x2 as u8, y2 as u8))
            })
        })
    }
}

fn in_bounds(x: i8, y: i8) -> bool {
    0 <= x && x < 40 && 0 <= y && y < 24
}
