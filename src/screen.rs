use crate::mem;

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
pub fn clear_lowres() {
    for i in 0x400..0x800 {
        unsafe { mem::write(i, 0) }
    }
}

/// A coordinate on the low-res (or text mode) screen.
///
/// Note that there's technically a "top-half" and a "bottom-half" to each of
/// these "pixels", in GR mode. Each can hold a 4-bit color.
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

    pub fn read(self) -> u8 {
        unsafe { self.addr().read_volatile() }
    }

    pub fn write(self, value: u8) {
        unsafe { self.addr().write_volatile(value) }
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
}
