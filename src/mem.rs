pub unsafe fn read(addr: u16) -> u8 {
    unsafe { (addr as *mut u8).read_volatile() }
}

pub unsafe fn write(addr: u16, val: u8) {
    unsafe { (addr as *mut u8).write_volatile(val) };
}
