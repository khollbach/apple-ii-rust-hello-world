#![no_std]

use panic_halt as _;

const LOW_RES_SCREEN: *mut u8 = 0x400 as _;
const TEXT_MODE_ON: *mut u8 = 0xc051 as _;

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    panic!("hi");
    unsafe { TEXT_MODE_ON.read_volatile() };
    unsafe { LOW_RES_SCREEN.write_volatile(b'A') };
    loop {}
}
