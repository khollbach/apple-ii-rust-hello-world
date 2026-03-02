#![no_std]

use core::{arch::asm, hint::black_box};

use panic_halt as _;

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    unsafe { (0xc050 as *mut u8).read_volatile(); } // gr
    unsafe { (0xc057 as *mut u8).read_volatile(); } // hires

    loop {
        for color in [0xff, 0x00] {
            for i in 0x2000..0x4000 {
                unsafe { (i as *mut u8).write_volatile(color); }

                for _ in 0..1_000 {
                    black_box(());
                }
            }
        }
    }

    loop {}
}
