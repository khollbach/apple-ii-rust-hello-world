#![no_std]

use panic_halt as _;

const LOW_RES_SCREEN: *mut u8 = 0x400 as _;
const TEXT_MODE_ON: *mut u8 = 0xc051 as _;

/*

Not yet working.

TODO: come back and figure out why.

[x] confirmed working: c2t + a single 'brk' instruction
[ ] next: inspect the blob that we're passing into c2t
        (disassemble it) -- what's supposed to happen if we run it?
    - if nothing jumps out at you, make a plan for how to
        test things out to get more information.

*/

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    unsafe { TEXT_MODE_ON.read_volatile() };
    unsafe { LOW_RES_SCREEN.write_volatile(b'A') };

    // for i in LOW_RES_SCREEN as u16..0x6000 {
    //     unsafe { (i as *mut u8).write_volatile(i as u8); }
    // }

    loop {}
}
