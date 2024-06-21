/* Apple II linker script, based on LLVM-MOS's `sim` linker script.
 *
 * Produces a memory image file. See LLVM-MOS's mos-sim for file format details.
 */

/* Provide imaginary (zero page) registers. */
__rc0 = 0x00;
INCLUDE imag-regs.ld
ASSERT(__rc31 == 0x001f, "Inconsistent zero page map.")

MEMORY {
    zp : ORIGIN = __rc31 + 1, LENGTH = 0x100 - (__rc31 + 1)

    /* KEVAN: our intention here is to tell the linker to put the program code
     * in the range $6000..$c000 -- this avoids all memory-mapped io locations.
     */
    /* ram (rw) : ORIGIN = 0x0200, LENGTH = 0xfdf0 */
    ram (rw) : ORIGIN = 0x6000, LENGTH = 0x6000
}

REGION_ALIAS("c_readonly", ram)
REGION_ALIAS("c_writeable", ram)

SECTIONS { INCLUDE c.ld }

/* KEVAN: let's make the stack grow down from $c000,
 * to avoid ROM and IO locations.
 */
/* Set initial soft stack address to just above last memory address. (It grows down.) */
/* __stack = 0xFFF0; */
__stack = 0xc000;

OUTPUT_FORMAT {
    /* KEVAN: load address should be $6000, not $200. */
    /* SHORT(0x0200) */
    /* SHORT(__data_end - 0x0200) */
    SHORT(0x6000)
    SHORT(__data_end - 0x6000)
    TRIM(ram)

    /* KEVAN: we'll leave this part as-is, and put a hack in the emulator to
     * detect the start address, based on this part of the memory image file.
     */
    SHORT(0xFFFA)
    SHORT(6)
    SHORT(0)
    SHORT(_start)
    SHORT(0)
}
