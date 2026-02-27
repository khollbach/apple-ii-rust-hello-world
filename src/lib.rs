#![no_std]

use core::hint::black_box;

use panic_halt as _;

const LOW_RES_SCREEN: *mut u8 = 0x400 as _;
const TEXT_MODE_ON: *mut u8 = 0xc051 as _;

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    // unsafe { TEXT_MODE_ON.read_volatile() };
    // unsafe { LOW_RES_SCREEN.write_volatile(b'A') };

    f();

    loop {}
}

#[inline(never)]
fn f() {
    let x = 5u8;
    black_box(x);
    g();
}

#[inline(never)]
fn g() {
    unsafe { TEXT_MODE_ON.read_volatile() };
    unsafe { LOW_RES_SCREEN.write_volatile(b'A') };
}
