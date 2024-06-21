#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // todo: something more useful
    loop {}
}

const LOW_RES_SCREEN: *mut u8 = 0x400 as _;
const TEXT_MODE_ON: *mut u8 = 0xc051 as _;

#[no_mangle]
extern "C" fn main() -> ! {
    unsafe { TEXT_MODE_ON.read_volatile() };
    unsafe { LOW_RES_SCREEN.write_volatile(b'A') };
    loop {}
}
